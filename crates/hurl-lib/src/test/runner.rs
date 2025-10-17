use crate::http::client::HttpClient;
use crate::http::request::RequestBuilder;
use crate::http::response::HttpResponse;
use crate::test::assertions::assert_response;
use crate::test::{TestCase, TestReport, TestResult, TestSuite};
use std::collections::HashMap;
use std::time::Instant;
use tokio::task::JoinHandle;

pub struct TestRunner {
    client: HttpClient,
    cache: TestResultCache,
}

pub struct TestResultCache {
    results: HashMap<String, TestResult>,
}

impl TestResultCache {
    fn new() -> Self {
        TestResultCache {
            results: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, result: TestResult) {
        self.results.insert(key, result);
    }

    fn get(&self, key: &str) -> Option<&TestResult> {
        self.results.get(key)
    }

    fn clear(&mut self) {
        self.results.clear();
    }
}

impl TestRunner {
    pub fn new() -> crate::Result<Self> {
        Ok(TestRunner {
            client: HttpClient::new()?,
            cache: TestResultCache::new(),
        })
    }

    pub fn run_test(&self, test: &TestCase) -> crate::Result<TestResult> {
        let start = Instant::now();
        let mut result = TestResult::new(test.name.clone());

        let mut request_builder = match test.request.method.as_str() {
            "GET" => RequestBuilder::get(&test.request.url),
            "POST" => RequestBuilder::post(&test.request.url),
            "PUT" => RequestBuilder::put(&test.request.url),
            "DELETE" => RequestBuilder::delete(&test.request.url),
            "PATCH" => RequestBuilder::patch(&test.request.url),
            "HEAD" => RequestBuilder::head(&test.request.url),
            "OPTIONS" => RequestBuilder::options(&test.request.url),
            _ => {
                result.set_error(format!("Unknown HTTP method: {}", test.request.method));
                return Ok(result);
            }
        };

        for (key, value) in &test.request.headers {
            request_builder = request_builder.header(key, value);
        }

        request_builder = request_builder.set_timeout(test.timeout);

        match self.execute_request(request_builder) {
            Ok(response) => {
                for assertion in &test.assertions {
                    let assertion_result = assert_response(&response, assertion);
                    result.add_assertion_result(assertion_result);
                }
            }
            Err(e) => {
                result.set_error(format!("Request failed: {}", e));
            }
        }

        let duration = start.elapsed();
        result.set_duration(duration);
        Ok(result)
    }

    pub fn run_suite(&mut self, suite: &TestSuite) -> crate::Result<TestReport> {
        let mut report = TestReport::new(suite.name.clone());

        for test in &suite.test_cases {
            let result = self.run_test(test)?;
            report.add_result(result);
        }

        report.finalize();
        Ok(report)
    }

    pub fn run_tests_parallel(&self, tests: &[TestCase]) -> crate::Result<Vec<TestResult>> {
        let runtime = tokio::runtime::Runtime::new()?;
        let results = runtime.block_on(self.run_parallel_async(tests))?;
        Ok(results)
    }

    async fn run_parallel_async(&self, tests: &[TestCase]) -> crate::Result<Vec<TestResult>> {
        let mut handles: Vec<JoinHandle<crate::Result<TestResult>>> = Vec::new();

        for test in tests {
            let test_clone = test.clone();
            let handle = tokio::spawn(async move {
                let runner = TestRunner::new()?;
                runner.run_test(&test_clone)
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            match handle.await {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => {
                    let mut result = TestResult::new("unknown".to_string());
                    result.set_error(format!("Parallel execution error: {}", e));
                    results.push(result);
                }
                Err(e) => {
                    let mut result = TestResult::new("unknown".to_string());
                    result.set_error(format!("Join error: {}", e));
                    results.push(result);
                }
            }
        }

        Ok(results)
    }

    pub fn filter_by_tag(&self, tests: &[TestCase], tag: &str) -> Vec<TestCase> {
        tests
            .iter()
            .filter(|t| t.tags.contains(&tag.to_string()))
            .cloned()
            .collect()
    }

    pub fn filter_by_name(&self, tests: &[TestCase], pattern: &str) -> Vec<TestCase> {
        tests
            .iter()
            .filter(|t| t.name.contains(pattern))
            .cloned()
            .collect()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    fn execute_request(&self, builder: RequestBuilder) -> crate::Result<HttpResponse> {
        let runtime = tokio::runtime::Runtime::new()?;
        runtime.block_on(async {
            let start = std::time::Instant::now();
            let response = self.client.execute(builder).await?;
            let duration = start.elapsed();
            Ok(response.with_duration(duration))
        })
    }
}

impl Default for TestRunner {
    fn default() -> Self {
        TestRunner::new().unwrap_or_else(|_| panic!("Failed to create TestRunner"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_runner_creation() {
        let runner = TestRunner::new();
        assert!(runner.is_ok());
    }

    #[test]
    fn test_test_result_cache_insert_get() {
        let mut cache = TestResultCache::new();
        let result = TestResult::new("test1".to_string());
        cache.insert("key1".to_string(), result);
        assert!(cache.get("key1").is_some());
    }

    #[test]
    fn test_test_result_cache_clear() {
        let mut cache = TestResultCache::new();
        let result = TestResult::new("test1".to_string());
        cache.insert("key1".to_string(), result);
        cache.clear();
        assert!(cache.get("key1").is_none());
    }

    #[test]
    fn test_filter_by_tag() {
        let runner = TestRunner::new().unwrap();
        let mut tests = vec![];

        let test1 = TestCase {
            name: "test1".to_string(),
            request: crate::test::TestRequest {
                method: "GET".to_string(),
                url: "https://example.com".to_string(),
                headers: HashMap::new(),
                body: None,
            },
            assertions: vec![],
            tags: vec!["smoke".to_string(), "api".to_string()],
            timeout: std::time::Duration::from_secs(30),
        };

        let test2 = TestCase {
            name: "test2".to_string(),
            request: crate::test::TestRequest {
                method: "GET".to_string(),
                url: "https://example.com".to_string(),
                headers: HashMap::new(),
                body: None,
            },
            assertions: vec![],
            tags: vec!["integration".to_string()],
            timeout: std::time::Duration::from_secs(30),
        };

        tests.push(test1);
        tests.push(test2);

        let filtered = runner.filter_by_tag(&tests, "smoke");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "test1");
    }

    #[test]
    fn test_filter_by_name() {
        let runner = TestRunner::new().unwrap();
        let tests = vec![
            TestCase {
                name: "test_get_users".to_string(),
                request: crate::test::TestRequest {
                    method: "GET".to_string(),
                    url: "https://example.com".to_string(),
                    headers: HashMap::new(),
                    body: None,
                },
                assertions: vec![],
                tags: vec![],
                timeout: std::time::Duration::from_secs(30),
            },
            TestCase {
                name: "test_post_user".to_string(),
                request: crate::test::TestRequest {
                    method: "POST".to_string(),
                    url: "https://example.com".to_string(),
                    headers: HashMap::new(),
                    body: None,
                },
                assertions: vec![],
                tags: vec![],
                timeout: std::time::Duration::from_secs(30),
            },
        ];

        let filtered = runner.filter_by_name(&tests, "get");
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_runner_default() {
        let _runner = TestRunner::default();
    }

    #[test]
    fn test_filter_by_tag_multiple_matches() {
        let runner = TestRunner::new().unwrap();
        let tests = vec![
            TestCase {
                name: "test1".to_string(),
                request: crate::test::TestRequest {
                    method: "GET".to_string(),
                    url: "https://example.com".to_string(),
                    headers: HashMap::new(),
                    body: None,
                },
                assertions: vec![],
                tags: vec!["smoke".to_string()],
                timeout: std::time::Duration::from_secs(30),
            },
            TestCase {
                name: "test2".to_string(),
                request: crate::test::TestRequest {
                    method: "GET".to_string(),
                    url: "https://example.com".to_string(),
                    headers: HashMap::new(),
                    body: None,
                },
                assertions: vec![],
                tags: vec!["smoke".to_string()],
                timeout: std::time::Duration::from_secs(30),
            },
        ];

        let filtered = runner.filter_by_tag(&tests, "smoke");
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_tag_no_matches() {
        let runner = TestRunner::new().unwrap();
        let tests = vec![TestCase {
            name: "test1".to_string(),
            request: crate::test::TestRequest {
                method: "GET".to_string(),
                url: "https://example.com".to_string(),
                headers: HashMap::new(),
                body: None,
            },
            assertions: vec![],
            tags: vec!["smoke".to_string()],
            timeout: std::time::Duration::from_secs(30),
        }];

        let filtered = runner.filter_by_tag(&tests, "integration");
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_by_name_no_matches() {
        let runner = TestRunner::new().unwrap();
        let tests = vec![TestCase {
            name: "test_get_users".to_string(),
            request: crate::test::TestRequest {
                method: "GET".to_string(),
                url: "https://example.com".to_string(),
                headers: HashMap::new(),
                body: None,
            },
            assertions: vec![],
            tags: vec![],
            timeout: std::time::Duration::from_secs(30),
        }];

        let filtered = runner.filter_by_name(&tests, "delete");
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_filter_by_name_multiple_matches() {
        let runner = TestRunner::new().unwrap();
        let tests = vec![
            TestCase {
                name: "test_users_get".to_string(),
                request: crate::test::TestRequest {
                    method: "GET".to_string(),
                    url: "https://example.com".to_string(),
                    headers: HashMap::new(),
                    body: None,
                },
                assertions: vec![],
                tags: vec![],
                timeout: std::time::Duration::from_secs(30),
            },
            TestCase {
                name: "test_users_post".to_string(),
                request: crate::test::TestRequest {
                    method: "POST".to_string(),
                    url: "https://example.com".to_string(),
                    headers: HashMap::new(),
                    body: None,
                },
                assertions: vec![],
                tags: vec![],
                timeout: std::time::Duration::from_secs(30),
            },
        ];

        let filtered = runner.filter_by_name(&tests, "users");
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_runner_cache_clear() {
        let mut runner = TestRunner::new().unwrap();
        runner.clear_cache();
    }
}
