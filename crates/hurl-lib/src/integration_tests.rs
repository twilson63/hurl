#[cfg(test)]
mod integration_tests {
    use crate::batch::{BatchExecutor, BatchRequest, BatchStats};
    use crate::http::chaining::{
        ChainContext, ChainRequest, ChainStep, ExtractionRule, Extractor, Variables,
    };
    use crate::http::request::RequestBuilder;
    use crate::http::response::HttpResponse;
    use crate::storage::cache::{CachePolicies, ResponseCache};
    use crate::storage::history::{HistoryEntry, HistoryStore, SearchQuery};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::time::Duration;

    fn create_test_request() -> RequestBuilder {
        RequestBuilder::get("https://api.example.com/users")
            .header("Content-Type", "application/json")
    }

    fn create_test_response(status: u16) -> HttpResponse {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        HttpResponse::new(status, headers, r#"{"id": 1, "name": "test"}"#.to_string())
            .with_duration(Duration::from_millis(100))
    }

    #[test]
    fn test_history_storage_complete_workflow() {
        let mut store = HistoryStore::new();
        let req = create_test_request();
        let resp = create_test_response(200);

        let entry =
            HistoryEntry::from_request_response(&req, &resp, vec!["api".to_string()]).unwrap();
        let id = store.save(entry);

        assert_eq!(store.len(), 1);
        assert!(store.get(&id).is_some());

        let query = SearchQuery::new().url("api.example.com").method("GET");
        let results = store.search(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, 200);
    }

    #[test]
    fn test_history_export_and_reimport() {
        let mut store = HistoryStore::new();
        for i in 0..3 {
            let req = RequestBuilder::get(format!("https://api.example.com/users/{}", i));
            let resp = create_test_response(200);
            let entry = HistoryEntry::from_request_response(&req, &resp, vec!["batch".to_string()])
                .unwrap();
            store.save(entry);
        }

        let json = store.export_to_json().unwrap();

        let mut new_store = HistoryStore::new();
        new_store.import_from_json(&json).unwrap();
        assert_eq!(new_store.len(), 3);
    }

    #[test]
    fn test_response_cache_basic() {
        let mut cache = ResponseCache::with_default_policies();
        let response = create_test_response(200);

        cache.put("https://api.example.com/users", "GET", response.clone());
        assert_eq!(cache.size(), 1);

        let cached = cache.get("https://api.example.com/users", "GET");
        assert!(cached.is_some());

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.size, 1);
    }

    #[test]
    fn test_response_cache_policies() {
        let policies = CachePolicies {
            default_ttl: Duration::from_secs(60),
            max_entries: 10,
            cache_by_method: true,
            cache_successful_only: false,
        };
        let mut cache = ResponseCache::new(policies);
        let response = create_test_response(500);

        cache.put("https://api.example.com", "GET", response);
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn test_variables_management() {
        let mut vars = Variables::new();
        vars.set("user_id", Value::Number(42.into()));
        vars.set("token", Value::String("abc123".to_string()));

        assert_eq!(vars.get_number("user_id"), Some(42));
        assert_eq!(vars.get_string("token"), Some("abc123".to_string()));

        let all = vars.all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_json_path_extraction() {
        let json = r#"{"user": {"id": 123, "name": "John", "items": [{"id": 1}, {"id": 2}]}}"#;

        let value = Extractor::extract_json_path(json, "user.name").unwrap();
        assert_eq!(value.as_str(), Some("John"));

        let value = Extractor::extract_json_path(json, "user.id").unwrap();
        assert_eq!(value.as_i64(), Some(123));

        let value = Extractor::extract_json_path(json, "user.items[0].id").unwrap();
        assert_eq!(value.as_i64(), Some(1));
    }

    #[test]
    fn test_header_extraction() {
        let mut headers = HashMap::new();
        headers.insert("x-auth-token".to_string(), "secret123".to_string());
        let response = HttpResponse::new(200, headers, "{}".to_string());

        let value = Extractor::extract_header(&response, "x-auth-token").unwrap();
        assert_eq!(value, "secret123");
    }

    #[test]
    fn test_extraction_rules_on_response() {
        let json = r#"{"id": 42, "status": "active"}"#;
        let mut headers = HashMap::new();
        headers.insert("x-request-id".to_string(), "req-123".to_string());
        let response = HttpResponse::new(201, headers, json.to_string())
            .with_duration(Duration::from_millis(250));

        let rules = vec![
            ExtractionRule::json_path("user_id", "id"),
            ExtractionRule::header("request_id", "x-request-id"),
            ExtractionRule::status("response_status"),
            ExtractionRule::duration("response_time"),
            ExtractionRule::size("body_size"),
        ];

        let vars = Extractor::apply_extractions(&response, &rules).unwrap();
        assert_eq!(vars.get_number("user_id"), Some(42));
        assert_eq!(vars.get_string("request_id"), Some("req-123".to_string()));
        assert_eq!(vars.get_number("response_status"), Some(201));
        assert_eq!(vars.get_number("response_time"), Some(250));
    }

    #[test]
    fn test_chain_request_variable_substitution() {
        let mut vars = Variables::new();
        vars.set("user_id", Value::String("123".to_string()));
        vars.set("api_key", Value::String("secret".to_string()));

        let req = ChainRequest::new("GET", "https://api.example.com/users/${user_id}")
            .header("Authorization", "Bearer ${api_key}");

        assert!(req.url_template.contains("${user_id}"));
        assert!(req.headers["Authorization"].contains("${api_key}"));
    }

    #[test]
    fn test_chain_context_operations() {
        let mut context = ChainContext::new();
        let response = create_test_response(200);

        let step = ChainStep {
            name: "fetch_user".to_string(),
            request: ChainRequest::new("GET", "https://api.example.com/users/1"),
            response,
            extracted_vars: HashMap::new(),
        };

        context.steps.push(step);

        assert!(context.get_step_by_name("fetch_user").is_some());
        assert_eq!(context.get_step(0).unwrap().name, "fetch_user");
    }

    #[test]
    fn test_batch_request_creation() {
        let mut batch = Vec::new();
        for i in 0..5 {
            let req = RequestBuilder::get(format!("https://api.example.com/item/{}", i));
            let batch_req = BatchRequest::new(&format!("req_{}", i), req);
            batch.push(batch_req);
        }
        assert_eq!(batch.len(), 5);
    }

    #[test]
    fn test_batch_executor_configuration() {
        let executor = BatchExecutor::new(20)
            .add_request(BatchRequest::new("req_1", create_test_request()))
            .add_request(BatchRequest::new("req_2", create_test_request()))
            .with_stop_on_error(true);

        assert_eq!(executor.request_count(), 2);
    }

    #[test]
    fn test_batch_stats_calculations() {
        let mut stats = BatchStats::default();
        stats.total = 20;
        stats.succeeded = 18;
        stats.failed = 2;
        stats.duration = Duration::from_secs(2);

        assert_eq!(stats.success_rate(), 90.0);
        assert_eq!(stats.error_rate(), 10.0);
        assert_eq!(stats.avg_time_per_request().as_millis(), 100);
    }

    #[test]
    fn test_search_query_combinations() {
        let mut store = HistoryStore::new();

        let req1 = RequestBuilder::get("https://api.example.com/users");
        let resp1 = create_test_response(200);
        let entry1 =
            HistoryEntry::from_request_response(&req1, &resp1, vec!["api".to_string()]).unwrap();
        store.save(entry1);

        let req2 = RequestBuilder::post("https://api.example.com/users");
        let resp2 = create_test_response(201);
        let entry2 =
            HistoryEntry::from_request_response(&req2, &resp2, vec!["api".to_string()]).unwrap();
        store.save(entry2);

        let query = SearchQuery::new()
            .url("example.com")
            .method("GET")
            .tags(vec!["api".to_string()]);

        let results = store.search(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].method, "GET");
    }

    #[test]
    fn test_history_multiple_operations() {
        let mut store = HistoryStore::new();

        for i in 0..10 {
            let req = RequestBuilder::get(format!("https://api.example.com/item/{}", i));
            let resp = create_test_response(if i % 2 == 0 { 200 } else { 404 });
            let entry = HistoryEntry::from_request_response(&req, &resp, vec![]).unwrap();
            store.save(entry);
        }

        assert_eq!(store.len(), 10);

        let query = SearchQuery::new().status(200);
        let successful = store.search(&query);
        assert_eq!(successful.len(), 5);

        let query = SearchQuery::new().status(404);
        let not_found = store.search(&query);
        assert_eq!(not_found.len(), 5);
    }

    #[test]
    fn test_cache_hit_rate_calculation() {
        let mut cache = ResponseCache::with_default_policies();
        let response = create_test_response(200);

        cache.put("https://api.example.com/1", "GET", response.clone());

        cache.get("https://api.example.com/1", "GET");
        cache.get("https://api.example.com/1", "GET");
        cache.get("https://api.example.com/1", "GET");
        cache.get("https://api.example.com/2", "GET");

        let stats = cache.stats();
        assert_eq!(stats.hits, 3);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate, 75.0);
    }

    #[test]
    fn test_cache_invalidation() {
        let mut cache = ResponseCache::with_default_policies();
        let response = create_test_response(200);

        cache.put("https://api.example.com/users", "GET", response.clone());
        cache.put("https://api.example.com/users", "POST", response);

        assert_eq!(cache.size(), 2);
        cache.invalidate("https://api.example.com/users", Some("GET"));
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn test_variables_all_types() {
        let mut vars = Variables::new();
        vars.set("string", Value::String("hello".to_string()));
        vars.set("number", Value::Number(42.into()));
        vars.set("bool", Value::Bool(true));
        vars.set("array", Value::Array(vec![Value::Number(1.into())]));

        assert!(vars.get_string("string").is_some());
        assert!(vars.get_number("number").is_some());
        assert!(vars.get_bool("bool").is_some());
        assert!(vars.get("array").is_some());
    }

    #[test]
    fn test_extraction_rule_builders() {
        let rules = vec![
            ExtractionRule::json_path("id", "data.id"),
            ExtractionRule::header("token", "x-token"),
            ExtractionRule::status("code"),
            ExtractionRule::duration("time"),
            ExtractionRule::size("length"),
        ];

        assert_eq!(rules.len(), 5);
        assert_eq!(rules[0].name, "id");
        assert_eq!(rules[1].name, "token");
        assert_eq!(rules[2].name, "code");
    }

    #[test]
    fn test_history_entry_with_multiple_tags() {
        let req = create_test_request();
        let resp = create_test_response(200);
        let tags = vec![
            "api".to_string(),
            "production".to_string(),
            "critical".to_string(),
        ];

        let entry = HistoryEntry::from_request_response(&req, &resp, tags.clone()).unwrap();
        assert_eq!(entry.tags.len(), 3);
        assert_eq!(entry.tags, tags);
    }

    #[test]
    fn test_chain_request_building() {
        let req = ChainRequest::new("POST", "https://api.example.com/users")
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer token")
            .body(r#"{"name": "John"}"#)
            .extract(ExtractionRule::json_path("user_id", "id"))
            .name("create_user");

        assert_eq!(req.method, "POST");
        assert_eq!(req.headers.len(), 2);
        assert_eq!(req.extractions.len(), 1);
        assert_eq!(req.name, Some("create_user".to_string()));
    }

    #[test]
    fn test_batch_metadata() {
        let batch_req = BatchRequest::new("req_1", create_test_request())
            .with_metadata("priority", "high")
            .with_metadata("retry", "3");

        assert_eq!(
            batch_req.metadata.get("priority"),
            Some(&"high".to_string())
        );
        assert_eq!(batch_req.metadata.get("retry"), Some(&"3".to_string()));
    }

    #[test]
    fn test_history_search_empty_result() {
        let store = HistoryStore::new();
        let query = SearchQuery::new().url("nonexistent.com");
        let results = store.search(&query);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_cache_clear_resets_stats() {
        let mut cache = ResponseCache::with_default_policies();
        let response = create_test_response(200);

        cache.put("https://api.example.com", "GET", response.clone());
        cache.get("https://api.example.com", "GET");
        cache.get("https://api.example.com", "GET");

        let stats_before = cache.stats();
        assert_eq!(stats_before.hits, 2);

        cache.clear();
        let stats_after = cache.stats();
        assert_eq!(stats_after.hits, 0);
        assert_eq!(stats_after.size, 0);
    }

    #[test]
    fn test_variables_clone_independence() {
        let mut vars1 = Variables::new();
        vars1.set("key", Value::String("value1".to_string()));

        let mut vars2 = vars1.clone();
        vars2.set("key", Value::String("value2".to_string()));

        assert_eq!(vars1.get_string("key"), Some("value1".to_string()));
        assert_eq!(vars2.get_string("key"), Some("value2".to_string()));
    }

    #[test]
    fn test_complex_json_path_navigation() {
        let json = r#"
        {
            "data": {
                "users": [
                    {"id": 1, "name": "Alice", "roles": ["admin"]},
                    {"id": 2, "name": "Bob", "roles": ["user"]}
                ]
            }
        }
        "#;

        let value = Extractor::extract_json_path(json, "data.users[0].name").unwrap();
        assert_eq!(value.as_str(), Some("Alice"));

        let value = Extractor::extract_json_path(json, "data.users[1].id").unwrap();
        assert_eq!(value.as_i64(), Some(2));
    }
}
