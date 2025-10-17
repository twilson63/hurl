use std::collections::HashMap;
use std::time::Duration;

mod common;

#[test]
fn test_basic_get_request_workflow() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/get";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client.get(url).send().await
    });
    
    assert!(resp.is_ok(), "Basic GET request should succeed");
    let response = resp.unwrap();
    assert_eq!(response.status(), 200, "Should return 200 OK");
}

#[test]
fn test_post_with_json_payload() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/post";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({"test": "data"}))
            .send()
            .await
    });
    
    assert!(resp.is_ok(), "POST with JSON should succeed");
    let response = resp.unwrap();
    assert_eq!(response.status(), 200);
}

#[test]
fn test_multiple_http_methods() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org";
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let methods = vec![
        ("GET", format!("{}/get", url)),
        ("POST", format!("{}/post", url)),
        ("PUT", format!("{}/put", url)),
        ("DELETE", format!("{}/delete", url)),
    ];
    
    for (method, test_url) in methods {
        let result = rt.block_on(async {
            match method {
                "GET" => client.get(&test_url).send().await,
                "POST" => client.post(&test_url).send().await,
                "PUT" => client.put(&test_url).send().await,
                "DELETE" => client.delete(&test_url).send().await,
                _ => unreachable!(),
            }
        });
        
        assert!(result.is_ok(), "{} request should succeed", method);
        assert_eq!(result.unwrap().status(), 200, "{} should return 200", method);
    }
}

#[test]
fn test_request_with_custom_headers() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/headers";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .get(url)
            .header("X-Custom-Header", "test-value")
            .header("User-Agent", "hurl/0.1.0")
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    let response = resp.unwrap();
    assert_eq!(response.status(), 200);
}

#[test]
fn test_query_parameter_handling() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/get";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .get(url)
            .query(&[("key1", "value1"), ("key2", "value2")])
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap().status(), 200);
}

#[test]
fn test_response_status_codes() {
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let test_cases = vec![
        ("https://httpbin.org/status/200", 200),
        ("https://httpbin.org/status/201", 201),
        ("https://httpbin.org/status/400", 400),
        ("https://httpbin.org/status/404", 404),
        ("https://httpbin.org/status/500", 500),
    ];
    
    for (url, expected_status) in test_cases {
        let resp = rt.block_on(async { client.get(url).send().await });
        assert!(resp.is_ok(), "Request to {} should succeed", url);
        assert_eq!(
            resp.unwrap().status().as_u16(),
            expected_status,
            "Status should be {}",
            expected_status
        );
    }
}

#[test]
fn test_response_headers_parsing() {
    let client = reqwest::Client::new();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client.get("https://httpbin.org/get").send().await
    });
    
    assert!(resp.is_ok());
    let response = resp.unwrap();
    
    assert!(
        response.headers().contains_key("content-type"),
        "Should contain content-type header"
    );
    assert!(
        response.headers().contains_key("content-length"),
        "Should contain content-length header"
    );
}

#[test]
fn test_json_response_parsing() {
    let client = reqwest::Client::new();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .post("https://httpbin.org/post")
            .json(&serde_json::json!({"key": "value"}))
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    let response = resp.unwrap();
    
    let json_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { response.json::<serde_json::Value>().await });
    
    assert!(json_result.is_ok(), "Should parse JSON response");
}

#[test]
fn test_timeout_configuration() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client.get("https://httpbin.org/delay/1").send().await
    });
    
    assert!(resp.is_ok(), "Request within timeout should succeed");
}

#[test]
fn test_redirect_following() {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .unwrap();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client.get("https://httpbin.org/redirect/2").send().await
    });
    
    assert!(resp.is_ok(), "Should follow redirects");
    assert_eq!(resp.unwrap().status(), 200);
}

#[test]
fn test_authentication_basic_auth() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/basic-auth/user/password";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .get(url)
            .basic_auth("user", Some("password"))
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap().status(), 200);
}

#[test]
fn test_response_body_text() {
    let client = reqwest::Client::new();
    
    let text_result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let resp = client.get("https://httpbin.org/get").send().await;
        resp.unwrap().text().await
    });
    
    assert!(text_result.is_ok());
    let body = text_result.unwrap();
    assert!(!body.is_empty(), "Response body should not be empty");
}

#[test]
fn test_response_body_bytes() {
    let client = reqwest::Client::new();
    
    let bytes_result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let resp = client.get("https://httpbin.org/bytes/100").send().await;
        resp.unwrap().bytes().await
    });
    
    assert!(bytes_result.is_ok());
    let body = bytes_result.unwrap();
    assert!(!body.is_empty(), "Response bytes should not be empty");
}

#[test]
fn test_concurrent_requests() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/get";
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let responses = rt.block_on(async {
        let futures: Vec<_> = (0..5)
            .map(|_| client.get(url).send())
            .collect();
        
        futures::future::join_all(futures).await
    });
    
    assert_eq!(responses.len(), 5);
    for response in responses {
        assert!(response.is_ok(), "All concurrent requests should succeed");
    }
}

#[test]
fn test_form_data_submission() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/post";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let mut form = HashMap::new();
        form.insert("field1", "value1");
        form.insert("field2", "value2");
        
        client.post(url).form(&form).send().await
    });
    
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap().status(), 200);
}

#[test]
fn test_error_handling_connection_refused() {
    let client = reqwest::Client::new();
    let url = "http://localhost:1/nonexistent";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client.get(url).send().await
    });
    
    assert!(resp.is_err(), "Request to refused port should fail");
}

#[test]
fn test_error_handling_invalid_url() {
    let result = reqwest::Url::parse("not a valid url");
    assert!(result.is_err(), "Invalid URL should fail to parse");
}

#[test]
fn test_response_size_tracking() {
    let client = reqwest::Client::new();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .get("https://httpbin.org/bytes/1024")
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    let response = resp.unwrap();
    
    let content_length = response
        .headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<usize>().ok());
    
    assert!(content_length.is_some(), "Should have content-length header");
}

#[test]
fn test_request_response_timing() {
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .get("https://httpbin.org/delay/1")
            .send()
            .await
    });
    
    let duration = start.elapsed();
    
    assert!(resp.is_ok());
    assert!(
        duration.as_secs() >= 1,
        "Request should take at least 1 second"
    );
}

#[test]
fn test_stress_multiple_sequential_requests() {
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut success_count = 0;
    for _i in 0..10 {
        let resp = rt.block_on(async {
            client.get("https://httpbin.org/get").send().await
        });
        
        if resp.is_ok() && resp.unwrap().status() == 200 {
            success_count += 1;
        }
    }
    
    assert_eq!(success_count, 10, "All 10 sequential requests should succeed");
}

#[test]
fn test_stress_concurrent_batch_requests() {
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let successful_responses = rt.block_on(async {
        let futures: Vec<_> = (0..20)
            .map(|_| {
                let c = client.clone();
                async move { c.get("https://httpbin.org/get").send().await }
            })
            .collect();
        
        let responses = futures::future::join_all(futures).await;
        responses
            .into_iter()
            .filter(|r| r.is_ok() && r.as_ref().unwrap().status() == 200)
            .count()
    });
    
    assert!(
        successful_responses >= 18,
        "At least 18 out of 20 concurrent requests should succeed"
    );
}

#[test]
fn test_response_decompression_gzip() {
    let client = reqwest::Client::new();
    let url = "https://httpbin.org/gzip";
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client.get(url).send().await
    });
    
    assert!(resp.is_ok());
    let response = resp.unwrap();
    assert_eq!(response.status(), 200);
}

#[test]
fn test_complex_workflow_authentication_and_parsing() {
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let result = rt.block_on(async {
        let response = client
            .get("https://httpbin.org/anything/test")
            .header("X-Test", "value")
            .send()
            .await;
        
        match response {
            Ok(resp) if resp.status() == 200 => {
                resp.json::<serde_json::Value>().await.ok()
            }
            _ => None,
        }
    });
    
    assert!(result.is_some(), "Complex workflow should complete successfully");
}

#[test]
fn test_memory_efficiency_large_response() {
    let client = reqwest::Client::new();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .get("https://httpbin.org/bytes/1000000")
            .send()
            .await
    });
    
    assert!(resp.is_ok(), "Should handle large responses");
}

#[test]
fn test_performance_request_latency() {
    let client = reqwest::Client::new();
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut times = Vec::new();
    for _ in 0..5 {
        let start = std::time::Instant::now();
        let _ = rt.block_on(async { client.get("https://httpbin.org/get").send().await });
        times.push(start.elapsed());
    }
    
    let avg_time = times.iter().sum::<Duration>() / times.len() as u32;
    assert!(
        avg_time.as_millis() < 2000,
        "Average request time should be reasonable: {:?}",
        avg_time
    );
}

#[test]
fn test_feature_combination_headers_and_json() {
    let client = reqwest::Client::new();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .post("https://httpbin.org/post")
            .header("X-Custom", "header-value")
            .json(&serde_json::json!({
                "field1": "value1",
                "nested": { "field2": "value2" }
            }))
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap().status(), 200);
}

#[test]
fn test_feature_combination_auth_and_timeout() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .get("https://httpbin.org/basic-auth/test/pass")
            .basic_auth("test", Some("pass"))
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    assert_eq!(resp.unwrap().status(), 200);
}

#[test]
fn test_multiple_feature_combinations() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .unwrap();
    
    let resp = tokio::runtime::Runtime::new().unwrap().block_on(async {
        client
            .post("https://httpbin.org/post")
            .header("X-Test-Header", "test-value")
            .query(&[("param", "value")])
            .json(&serde_json::json!({"data": "test"}))
            .send()
            .await
    });
    
    assert!(resp.is_ok());
    let response = resp.unwrap();
    assert_eq!(response.status(), 200);
    
    let json = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { response.json::<serde_json::Value>().await });
    
    assert!(json.is_ok());
}
