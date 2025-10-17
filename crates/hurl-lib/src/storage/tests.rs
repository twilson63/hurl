#[cfg(test)]
mod tests {
    use crate::storage::history::*;
    use crate::http::request::RequestBuilder;
    use crate::http::response::HttpResponse;
    use std::collections::HashMap;
    use std::time::Duration;

    fn create_test_request() -> RequestBuilder {
        RequestBuilder::get("https://api.example.com/users")
            .header("Content-Type", "application/json")
    }

    fn create_test_response() -> HttpResponse {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        HttpResponse::new(
            200,
            headers,
            r#"{"id": 1, "name": "test"}"#.to_string(),
        )
        .with_duration(Duration::from_millis(100))
    }

    #[test]
    fn test_history_entry_creation() {
        let req = create_test_request();
        let resp = create_test_response();
        let entry = HistoryEntry::from_request_response(&req, &resp, vec!["api".to_string()]);
        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.url, "https://api.example.com/users");
        assert_eq!(entry.method, "GET");
        assert_eq!(entry.status, 200);
        assert_eq!(entry.tags, vec!["api".to_string()]);
    }

    #[test]
    fn test_history_store_save_and_get() {
        let mut store = HistoryStore::new();
        let req = create_test_request();
        let resp = create_test_response();
        let entry = HistoryEntry::from_request_response(&req, &resp, vec![]).unwrap();
        let id = store.save(entry);
        
        let retrieved = store.get(&id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().url, "https://api.example.com/users");
    }

    #[test]
    fn test_history_store_search_by_url() {
        let mut store = HistoryStore::new();
        let req = create_test_request();
        let resp = create_test_response();
        let entry = HistoryEntry::from_request_response(&req, &resp, vec![]).unwrap();
        store.save(entry);

        let query = SearchQuery::new().url("api.example.com");
        let results = store.search(&query);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_history_store_search_by_method() {
        let mut store = HistoryStore::new();
        let req = create_test_request();
        let resp = create_test_response();
        let entry = HistoryEntry::from_request_response(&req, &resp, vec![]).unwrap();
        store.save(entry);

        let query = SearchQuery::new().method("GET");
        let results = store.search(&query);
        assert_eq!(results.len(), 1);

        let query = SearchQuery::new().method("POST");
        let results = store.search(&query);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_history_store_delete() {
        let mut store = HistoryStore::new();
        let req = create_test_request();
        let resp = create_test_response();
        let entry = HistoryEntry::from_request_response(&req, &resp, vec![]).unwrap();
        let id = store.save(entry);
        assert_eq!(store.len(), 1);
        
        let deleted = store.delete(&id);
        assert!(deleted);
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_history_store_export_import() {
        let mut store = HistoryStore::new();
        let req = create_test_request();
        let resp = create_test_response();
        let entry = HistoryEntry::from_request_response(&req, &resp, vec![]).unwrap();
        store.save(entry);

        let json = store.export_to_json().unwrap();
        assert!(!json.is_empty());

        let mut new_store = HistoryStore::new();
        new_store.import_from_json(&json).unwrap();
        assert_eq!(new_store.len(), 1);
    }

    #[test]
    fn test_history_store_clear() {
        let mut store = HistoryStore::new();
        let req = create_test_request();
        let resp = create_test_response();
        let entry = HistoryEntry::from_request_response(&req, &resp, vec![]).unwrap();
        store.save(entry);
        
        store.clear();
        assert_eq!(store.len(), 0);
    }
}
