use crate::http::response::HttpResponse;
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Assertion {
    StatusCode(u16),
    StatusRange {
        min: u16,
        max: u16,
    },
    HeaderExists(String),
    HeaderValue {
        name: String,
        expected: String,
    },
    BodyContains(String),
    BodyRegex(String),
    JsonPath {
        path: String,
        expected: Value,
    },
    JsonType {
        path: String,
        expected_type: JsonType,
    },
    ResponseTime {
        max_ms: u64,
    },
    ResponseSize {
        min: usize,
        max: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonType {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}

impl std::fmt::Display for JsonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonType::Object => write!(f, "object"),
            JsonType::Array => write!(f, "array"),
            JsonType::String => write!(f, "string"),
            JsonType::Number => write!(f, "number"),
            JsonType::Boolean => write!(f, "boolean"),
            JsonType::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AssertionResult {
    pub passed: bool,
    pub assertion: String,
    pub expected: String,
    pub actual: String,
    pub message: String,
    pub suggestion: Option<String>,
}

impl AssertionResult {
    pub fn new(
        passed: bool,
        assertion: String,
        expected: String,
        actual: String,
        message: String,
    ) -> Self {
        let suggestion = if !passed {
            Some(Self::generate_suggestion(&assertion, &actual))
        } else {
            None
        };

        AssertionResult {
            passed,
            assertion,
            expected,
            actual,
            message,
            suggestion,
        }
    }

    fn generate_suggestion(assertion: &str, _actual: &str) -> String {
        match assertion {
            s if s.contains("status") => {
                "Check that the endpoint is responding with the correct status code".to_string()
            }
            s if s.contains("header") => "Verify the header is present in the response".to_string(),
            s if s.contains("body") => {
                "Ensure the response body contains the expected content".to_string()
            }
            s if s.contains("json") => "Validate the JSON structure and values".to_string(),
            s if s.contains("time") => "Check server performance or network latency".to_string(),
            s if s.contains("size") => {
                "Verify the response size is within expected bounds".to_string()
            }
            _ => "Check the assertion criteria".to_string(),
        }
    }

    pub fn format_failure(&self) -> String {
        let mut output = format!(
            "FAILED: {}\nExpected: {}\nActual: {}",
            self.message, self.expected, self.actual
        );

        if let Some(ref suggestion) = self.suggestion {
            output.push_str(&format!("\nSuggestion: {}", suggestion));
        }

        output
    }
}

pub fn assert_response(response: &HttpResponse, assertion: &Assertion) -> AssertionResult {
    match assertion {
        Assertion::StatusCode(expected) => assert_status_code(response, *expected),
        Assertion::StatusRange { min, max } => assert_status_range(response, *min, *max),
        Assertion::HeaderExists(name) => assert_header_exists(response, name),
        Assertion::HeaderValue { name, expected } => assert_header_value(response, name, expected),
        Assertion::BodyContains(text) => assert_body_contains(response, text),
        Assertion::BodyRegex(pattern) => assert_body_regex(response, pattern),
        Assertion::JsonPath { path, expected } => assert_json_path(response, path, expected),
        Assertion::JsonType {
            path,
            expected_type,
        } => assert_json_type(response, path, *expected_type),
        Assertion::ResponseTime { max_ms } => assert_response_time(response, *max_ms),
        Assertion::ResponseSize { min, max } => assert_response_size(response, *min, *max),
    }
}

fn assert_status_code(response: &HttpResponse, expected: u16) -> AssertionResult {
    let actual = response.status;
    let passed = actual == expected;

    AssertionResult::new(
        passed,
        "status_code".to_string(),
        expected.to_string(),
        actual.to_string(),
        format!(
            "Status code assertion failed: expected {}, got {}",
            expected, actual
        ),
    )
}

fn assert_status_range(response: &HttpResponse, min: u16, max: u16) -> AssertionResult {
    let actual = response.status;
    let passed = actual >= min && actual <= max;

    AssertionResult::new(
        passed,
        "status_range".to_string(),
        format!("{}-{}", min, max),
        actual.to_string(),
        format!(
            "Status code range assertion failed: expected {}-{}, got {}",
            min, max, actual
        ),
    )
}

fn assert_header_exists(response: &HttpResponse, name: &str) -> AssertionResult {
    let exists = response.header(name).is_some();
    let actual = if exists { "found" } else { "not found" };

    AssertionResult::new(
        exists,
        "header_exists".to_string(),
        name.to_string(),
        actual.to_string(),
        format!("Header '{}' not found in response", name),
    )
}

fn assert_header_value(response: &HttpResponse, name: &str, expected: &str) -> AssertionResult {
    let actual = response
        .header(name)
        .unwrap_or("header not found")
        .to_string();
    let passed = response.header(name) == Some(expected);

    let message = format!(
        "Header '{}' value mismatch: expected '{}', got '{}'",
        name, expected, actual
    );

    AssertionResult::new(
        passed,
        "header_value".to_string(),
        expected.to_string(),
        actual,
        message,
    )
}

fn assert_body_contains(response: &HttpResponse, text: &str) -> AssertionResult {
    let passed = response.body.contains(text);

    AssertionResult::new(
        passed,
        "body_contains".to_string(),
        text.to_string(),
        if passed { "found" } else { "not found" }.to_string(),
        format!("Response body does not contain: '{}'", text),
    )
}

fn assert_body_regex(response: &HttpResponse, pattern: &str) -> AssertionResult {
    let regex_result = Regex::new(pattern);

    match regex_result {
        Ok(regex) => {
            let passed = regex.is_match(&response.body);
            AssertionResult::new(
                passed,
                "body_regex".to_string(),
                pattern.to_string(),
                if passed { "matched" } else { "no match" }.to_string(),
                format!("Response body does not match regex: '{}'", pattern),
            )
        }
        Err(e) => AssertionResult::new(
            false,
            "body_regex".to_string(),
            pattern.to_string(),
            format!("regex error: {}", e),
            format!("Invalid regex pattern: '{}'", pattern),
        ),
    }
}

fn assert_json_path(response: &HttpResponse, path: &str, expected: &Value) -> AssertionResult {
    match serde_json::from_str::<Value>(&response.body) {
        Ok(json) => {
            let actual_value = get_json_value(&json, path);
            let passed = actual_value == Some(expected.clone());

            let actual_str = actual_value
                .map(|v| v.to_string())
                .unwrap_or_else(|| "path not found".to_string());

            AssertionResult::new(
                passed,
                "json_path".to_string(),
                format!("{}: {}", path, expected),
                actual_str,
                format!("JSON path '{}' value mismatch", path),
            )
        }
        Err(e) => AssertionResult::new(
            false,
            "json_path".to_string(),
            path.to_string(),
            format!("json parse error: {}", e),
            format!("Response body is not valid JSON: {}", e),
        ),
    }
}

fn assert_json_type(
    response: &HttpResponse,
    path: &str,
    expected_type: JsonType,
) -> AssertionResult {
    match serde_json::from_str::<Value>(&response.body) {
        Ok(json) => {
            let value = get_json_value(&json, path);
            let actual_type = value.as_ref().map(determine_json_type);
            let passed = actual_type == Some(expected_type);

            let type_str = actual_type
                .map(|t| t.to_string())
                .unwrap_or_else(|| "path not found".to_string());

            AssertionResult::new(
                passed,
                "json_type".to_string(),
                expected_type.to_string(),
                type_str,
                format!("JSON path '{}' type mismatch", path),
            )
        }
        Err(e) => AssertionResult::new(
            false,
            "json_type".to_string(),
            path.to_string(),
            format!("json parse error: {}", e),
            format!("Response body is not valid JSON: {}", e),
        ),
    }
}

fn assert_response_time(response: &HttpResponse, max_ms: u64) -> AssertionResult {
    let actual_ms = response.duration.as_millis() as u64;
    let passed = actual_ms <= max_ms;

    AssertionResult::new(
        passed,
        "response_time".to_string(),
        format!("{}ms", max_ms),
        format!("{}ms", actual_ms),
        format!(
            "Response time exceeded: expected <= {}ms, got {}ms",
            max_ms, actual_ms
        ),
    )
}

fn assert_response_size(response: &HttpResponse, min: usize, max: usize) -> AssertionResult {
    let actual_size = response.body.len();
    let passed = actual_size >= min && actual_size <= max;

    AssertionResult::new(
        passed,
        "response_size".to_string(),
        format!("{}-{} bytes", min, max),
        format!("{} bytes", actual_size),
        format!(
            "Response size out of range: expected {}-{} bytes, got {}",
            min, max, actual_size
        ),
    )
}

fn get_json_value(json: &Value, path: &str) -> Option<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = json.clone();

    for part in parts {
        current = match current.get(part) {
            Some(v) => v.clone(),
            None => return None,
        };
    }

    Some(current)
}

fn determine_json_type(value: &Value) -> JsonType {
    match value {
        Value::Object(_) => JsonType::Object,
        Value::Array(_) => JsonType::Array,
        Value::String(_) => JsonType::String,
        Value::Number(_) => JsonType::Number,
        Value::Bool(_) => JsonType::Boolean,
        Value::Null => JsonType::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;

    fn create_response(status: u16, body: &str) -> HttpResponse {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        HttpResponse::new(status, headers, body.to_string())
            .with_duration(Duration::from_millis(100))
    }

    #[test]
    fn test_status_code_assertion_pass() {
        let response = create_response(200, "{}");
        let assertion = Assertion::StatusCode(200);
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_status_code_assertion_fail() {
        let response = create_response(404, "{}");
        let assertion = Assertion::StatusCode(200);
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_status_range_assertion_pass() {
        let response = create_response(201, "{}");
        let assertion = Assertion::StatusRange { min: 200, max: 299 };
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_status_range_assertion_fail() {
        let response = create_response(500, "{}");
        let assertion = Assertion::StatusRange { min: 200, max: 299 };
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_header_exists_assertion_pass() {
        let response = create_response(200, "{}");
        let assertion = Assertion::HeaderExists("Content-Type".to_string());
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_header_exists_assertion_fail() {
        let response = create_response(200, "{}");
        let assertion = Assertion::HeaderExists("X-Custom-Header".to_string());
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_header_value_assertion_pass() {
        let response = create_response(200, "{}");
        let assertion = Assertion::HeaderValue {
            name: "Content-Type".to_string(),
            expected: "application/json".to_string(),
        };
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_header_value_assertion_fail() {
        let response = create_response(200, "{}");
        let assertion = Assertion::HeaderValue {
            name: "Content-Type".to_string(),
            expected: "text/plain".to_string(),
        };
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_body_contains_assertion_pass() {
        let response = create_response(200, r#"{"status":"ok"}"#);
        let assertion = Assertion::BodyContains("ok".to_string());
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_body_contains_assertion_fail() {
        let response = create_response(200, r#"{"status":"ok"}"#);
        let assertion = Assertion::BodyContains("error".to_string());
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_body_regex_assertion_pass() {
        let response = create_response(200, r#"{"status":"ok"}"#);
        let assertion = Assertion::BodyRegex(r#""status":\s*"ok""#.to_string());
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_body_regex_assertion_fail() {
        let response = create_response(200, r#"{"status":"ok"}"#);
        let assertion = Assertion::BodyRegex(r#""status":\s*"error""#.to_string());
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_json_path_assertion_pass() {
        let response = create_response(200, r#"{"user":{"name":"John","age":30}}"#);
        let assertion = Assertion::JsonPath {
            path: "user.name".to_string(),
            expected: Value::String("John".to_string()),
        };
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_json_path_assertion_fail() {
        let response = create_response(200, r#"{"user":{"name":"John","age":30}}"#);
        let assertion = Assertion::JsonPath {
            path: "user.name".to_string(),
            expected: Value::String("Jane".to_string()),
        };
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_json_type_assertion_pass() {
        let response = create_response(200, r#"{"items":[1,2,3]}"#);
        let assertion = Assertion::JsonType {
            path: "items".to_string(),
            expected_type: JsonType::Array,
        };
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_json_type_assertion_fail() {
        let response = create_response(200, r#"{"items":"string"}"#);
        let assertion = Assertion::JsonType {
            path: "items".to_string(),
            expected_type: JsonType::Array,
        };
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_response_time_assertion_pass() {
        let response = create_response(200, "{}");
        let assertion = Assertion::ResponseTime { max_ms: 200 };
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_response_time_assertion_fail() {
        let response = create_response(200, "{}");
        let assertion = Assertion::ResponseTime { max_ms: 50 };
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_response_size_assertion_pass() {
        let response = create_response(200, r#"{"status":"ok"}"#);
        let assertion = Assertion::ResponseSize { min: 10, max: 100 };
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_response_size_assertion_fail() {
        let response = create_response(200, r#"{"status":"ok"}"#);
        let assertion = Assertion::ResponseSize {
            min: 100,
            max: 1000,
        };
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_assertion_result_has_suggestion_on_failure() {
        let response = create_response(200, "{}");
        let assertion = Assertion::StatusCode(500);
        let result = assert_response(&response, &assertion);
        assert!(!result.passed);
        assert!(result.suggestion.is_some());
    }

    #[test]
    fn test_assertion_result_no_suggestion_on_pass() {
        let response = create_response(200, "{}");
        let assertion = Assertion::StatusCode(200);
        let result = assert_response(&response, &assertion);
        assert!(result.passed);
        assert!(result.suggestion.is_none());
    }
}
