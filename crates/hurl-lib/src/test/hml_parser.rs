use crate::test::{Assertion, TestCase, TestRequest};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
    pub context: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse error at line {}: {}\nContext: {}",
            self.line, self.message, self.context
        )
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct HmlParser {
    lines: Vec<String>,
}

impl HmlParser {
    pub fn new(content: &str) -> Self {
        HmlParser {
            lines: content.lines().map(|l| l.to_string()).collect(),
        }
    }

    pub fn parse(&self) -> ParseResult<Vec<TestCase>> {
        let mut test_cases = Vec::new();
        let mut current_test: Option<TestCaseBuilder> = None;
        let mut i = 0;

        while i < self.lines.len() {
            let line = &self.lines[i];
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                i += 1;
                continue;
            }

            if trimmed.starts_with("@test") {
                if let Some(test) = current_test {
                    test_cases.push(test.build()?);
                }

                let test_name = trimmed
                    .strip_prefix("@test")
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if test_name.is_empty() {
                    return Err(ParseError {
                        line: i + 1,
                        message: "Test name required".to_string(),
                        context: line.clone(),
                    });
                }

                current_test = Some(TestCaseBuilder::new(test_name));
            } else if trimmed.starts_with("GET")
                || trimmed.starts_with("POST")
                || trimmed.starts_with("PUT")
                || trimmed.starts_with("DELETE")
                || trimmed.starts_with("PATCH")
                || trimmed.starts_with("HEAD")
            {
                if let Some(ref mut test) = current_test {
                    let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
                    if parts.len() < 2 {
                        return Err(ParseError {
                            line: i + 1,
                            message: "URL required for request".to_string(),
                            context: line.clone(),
                        });
                    }

                    test.set_method(parts[0].to_string());
                    test.set_url(parts[1].trim().to_string());
                } else {
                    return Err(ParseError {
                        line: i + 1,
                        message: "Request must be inside a @test section".to_string(),
                        context: line.clone(),
                    });
                }
            } else if trimmed.starts_with("assert_status:") {
                if let Some(ref mut test) = current_test {
                    let status_str = trimmed.strip_prefix("assert_status:").unwrap_or("").trim();
                    let status: u16 = status_str.parse().map_err(|_| ParseError {
                        line: i + 1,
                        message: format!("Invalid status code: {}", status_str),
                        context: line.clone(),
                    })?;
                    test.add_assertion(Assertion::StatusCode(status));
                }
            } else if trimmed.starts_with("assert_header:") {
                if let Some(ref mut test) = current_test {
                    let header_spec = trimmed.strip_prefix("assert_header:").unwrap_or("").trim();
                    let parts: Vec<&str> = header_spec.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        test.add_assertion(Assertion::HeaderValue {
                            name: parts[0].trim().to_string(),
                            expected: parts[1].trim().to_string(),
                        });
                    } else {
                        test.add_assertion(Assertion::HeaderExists(header_spec.to_string()));
                    }
                }
            } else if trimmed.starts_with("assert_body:") {
                if let Some(ref mut test) = current_test {
                    let body_content = trimmed.strip_prefix("assert_body:").unwrap_or("").trim();
                    test.add_assertion(Assertion::BodyContains(body_content.to_string()));
                }
            } else if trimmed.starts_with("assert_regex:") {
                if let Some(ref mut test) = current_test {
                    let pattern = trimmed.strip_prefix("assert_regex:").unwrap_or("").trim();
                    test.add_assertion(Assertion::BodyRegex(pattern.to_string()));
                }
            } else if trimmed.starts_with("assert_json:") {
                if let Some(ref mut test) = current_test {
                    let json_spec = trimmed.strip_prefix("assert_json:").unwrap_or("").trim();
                    let parts: Vec<&str> = json_spec.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        let path = parts[0].trim().to_string();
                        let value_str = parts[1].trim();

                        let expected_value = match value_str {
                            "true" => Value::Bool(true),
                            "false" => Value::Bool(false),
                            "null" => Value::Null,
                            s if s.starts_with('"') && s.ends_with('"') => {
                                Value::String(s[1..s.len() - 1].to_string())
                            }
                            s if s.parse::<i64>().is_ok() => {
                                Value::Number(s.parse::<i64>().unwrap().into())
                            }
                            s => Value::String(s.to_string()),
                        };

                        test.add_assertion(Assertion::JsonPath {
                            path,
                            expected: expected_value,
                        });
                    }
                }
            } else if trimmed.starts_with("assert_time:") {
                if let Some(ref mut test) = current_test {
                    let time_str = trimmed.strip_prefix("assert_time:").unwrap_or("").trim();
                    let max_ms: u64 = time_str.parse().map_err(|_| ParseError {
                        line: i + 1,
                        message: format!("Invalid time value: {}", time_str),
                        context: line.clone(),
                    })?;
                    test.add_assertion(Assertion::ResponseTime { max_ms });
                }
            } else if trimmed.starts_with("@tag:") {
                if let Some(ref mut test) = current_test {
                    let tag = trimmed.strip_prefix("@tag:").unwrap_or("").trim();
                    test.add_tag(tag.to_string());
                }
            }

            i += 1;
        }

        if let Some(test) = current_test {
            test_cases.push(test.build()?);
        }

        Ok(test_cases)
    }
}

struct TestCaseBuilder {
    name: String,
    method: Option<String>,
    url: Option<String>,
    headers: HashMap<String, String>,
    body: Option<String>,
    assertions: Vec<Assertion>,
    tags: Vec<String>,
    timeout: Duration,
}

impl TestCaseBuilder {
    fn new(name: String) -> Self {
        TestCaseBuilder {
            name,
            method: None,
            url: None,
            headers: HashMap::new(),
            body: None,
            assertions: Vec::new(),
            tags: Vec::new(),
            timeout: Duration::from_secs(30),
        }
    }

    fn set_method(&mut self, method: String) {
        self.method = Some(method);
    }

    fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    fn add_assertion(&mut self, assertion: Assertion) {
        self.assertions.push(assertion);
    }

    fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    fn build(self) -> ParseResult<TestCase> {
        let method = self.method.ok_or(ParseError {
            line: 0,
            message: format!("Test '{}': HTTP method not specified", self.name),
            context: String::new(),
        })?;

        let url = self.url.ok_or(ParseError {
            line: 0,
            message: format!("Test '{}': URL not specified", self.name),
            context: String::new(),
        })?;

        Ok(TestCase {
            name: self.name,
            request: TestRequest {
                method,
                url,
                headers: self.headers,
                body: self.body,
            },
            assertions: self.assertions,
            tags: self.tags,
            timeout: self.timeout,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_test() {
        let content = r#"
@test GetUser
GET https://api.example.com/users/1
assert_status: 200
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].name, "GetUser");
        assert_eq!(tests[0].request.method, "GET");
        assert_eq!(tests[0].request.url, "https://api.example.com/users/1");
    }

    #[test]
    fn test_parse_multiple_tests() {
        let content = r#"
@test Test1
GET https://example.com
assert_status: 200

@test Test2
POST https://example.com
assert_status: 201
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0].name, "Test1");
        assert_eq!(tests[1].name, "Test2");
    }

    #[test]
    fn test_parse_with_tags() {
        let content = r#"
@test GetUser
@tag: smoke
@tag: api
GET https://example.com
assert_status: 200
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].tags.len(), 2);
        assert!(tests[0].tags.contains(&"smoke".to_string()));
    }

    #[test]
    fn test_parse_status_assertion() {
        let content = r#"
@test Test
GET https://example.com
assert_status: 404
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_header_assertion() {
        let content = r#"
@test Test
GET https://example.com
assert_header: Content-Type=application/json
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_body_assertion() {
        let content = r#"
@test Test
GET https://example.com
assert_body: success
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_regex_assertion() {
        let content = r#"
@test Test
GET https://example.com
assert_regex: \d{3}
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_json_assertion() {
        let content = r#"
@test Test
GET https://example.com
assert_json: user.name="John"
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_time_assertion() {
        let content = r#"
@test Test
GET https://example.com
assert_time: 1000
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_multiple_assertions() {
        let content = r#"
@test Test
GET https://example.com
assert_status: 200
assert_body: success
assert_time: 5000
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 3);
    }

    #[test]
    fn test_parse_comments_ignored() {
        let content = r#"
# This is a comment
@test Test
GET https://example.com
# Another comment
assert_status: 200
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests.len(), 1);
    }

    #[test]
    fn test_parse_all_http_methods() {
        let methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"];
        for method in methods {
            let content = format!(
                "@test Test\n{} https://example.com\nassert_status: 200",
                method
            );
            let parser = HmlParser::new(&content);
            let tests = parser.parse().unwrap();
            assert_eq!(tests[0].request.method, method);
        }
    }

    #[test]
    fn test_parse_json_boolean_value() {
        let content = r#"
@test Test
GET https://example.com
assert_json: active=true
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_json_number_value() {
        let content = r#"
@test Test
GET https://example.com
assert_json: count=42
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].assertions.len(), 1);
    }

    #[test]
    fn test_parse_missing_method() {
        let content = r#"
@test Test
assert_status: 200
"#;

        let parser = HmlParser::new(content);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_url() {
        let content = r#"
@test Test
GET
assert_status: 200
"#;

        let parser = HmlParser::new(content);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_test_name() {
        let content = r#"
@test
GET https://example.com
"#;

        let parser = HmlParser::new(content);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let content = r#"
@test   TestName   
GET   https://example.com   
assert_status:   200   
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests[0].name, "TestName");
    }

    #[test]
    fn test_parse_empty_lines() {
        let content = r#"

@test Test1


GET https://example.com

assert_status: 200


@test Test2
GET https://example.com
"#;

        let parser = HmlParser::new(content);
        let tests = parser.parse().unwrap();
        assert_eq!(tests.len(), 2);
    }
}
