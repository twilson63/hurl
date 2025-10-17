pub mod assertions;
pub mod hml_parser;
pub mod runner;

use assertions::{Assertion, AssertionResult};
use chrono::Local;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub request: TestRequest,
    pub assertions: Vec<Assertion>,
    pub tags: Vec<String>,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct TestRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub passed: bool,
    pub duration: Duration,
    pub assertions_passed: usize,
    pub assertions_total: usize,
    pub assertion_results: Vec<AssertionResult>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl TestResult {
    pub fn new(test_name: String) -> Self {
        TestResult {
            test_name,
            passed: true,
            duration: Duration::from_secs(0),
            assertions_passed: 0,
            assertions_total: 0,
            assertion_results: Vec::new(),
            error: None,
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn add_assertion_result(&mut self, result: AssertionResult) {
        self.assertions_total += 1;
        if result.passed {
            self.assertions_passed += 1;
        } else {
            self.passed = false;
        }
        self.assertion_results.push(result);
    }

    pub fn set_error(&mut self, error: String) {
        self.passed = false;
        self.error = Some(error);
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn summary(&self) -> String {
        format!(
            "{}: {}/{} assertions passed in {:?}",
            if self.passed { "✓ PASS" } else { "✗ FAIL" },
            self.assertions_passed,
            self.assertions_total,
            self.duration
        )
    }
}

#[derive(Debug, Clone)]
pub struct TestSuite {
    pub name: String,
    pub test_cases: Vec<TestCase>,
}

impl TestSuite {
    pub fn new(name: impl Into<String>) -> Self {
        TestSuite {
            name: name.into(),
            test_cases: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test_case: TestCase) {
        self.test_cases.push(test_case);
    }

    pub fn add_tests(&mut self, test_cases: Vec<TestCase>) {
        self.test_cases.extend(test_cases);
    }
}

#[derive(Debug)]
pub struct TestReport {
    pub suite_name: String,
    pub results: Vec<TestResult>,
    pub total_duration: Duration,
    pub start_time: String,
    pub end_time: String,
}

impl TestReport {
    pub fn new(suite_name: String) -> Self {
        TestReport {
            suite_name,
            results: Vec::new(),
            total_duration: Duration::from_secs(0),
            start_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            end_time: String::new(),
        }
    }

    pub fn add_result(&mut self, result: TestResult) {
        self.results.push(result);
    }

    pub fn finalize(&mut self) {
        self.end_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

    pub fn total_tests(&self) -> usize {
        self.results.len()
    }

    pub fn passed_tests(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    pub fn failed_tests(&self) -> usize {
        self.total_tests() - self.passed_tests()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tests() == 0 {
            100.0
        } else {
            (self.passed_tests() as f64 / self.total_tests() as f64) * 100.0
        }
    }

    pub fn total_assertions(&self) -> usize {
        self.results.iter().map(|r| r.assertions_total).sum()
    }

    pub fn passed_assertions(&self) -> usize {
        self.results.iter().map(|r| r.assertions_passed).sum()
    }

    pub fn summary(&self) -> String {
        format!(
            "Test Suite: {}\n\
             Total Tests: {}\n\
             Passed: {}\n\
             Failed: {}\n\
             Success Rate: {:.1}%\n\
             Total Assertions: {}\n\
             Passed Assertions: {}\n\
             Total Duration: {:?}\n\
             Started: {}\n\
             Ended: {}",
            self.suite_name,
            self.total_tests(),
            self.passed_tests(),
            self.failed_tests(),
            self.success_rate(),
            self.total_assertions(),
            self.passed_assertions(),
            self.total_duration,
            self.start_time,
            self.end_time
        )
    }

    pub fn generate_html_report(&self) -> String {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Test Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }
        .header { background: #333; color: white; padding: 20px; border-radius: 5px; }
        .summary { background: white; padding: 15px; margin: 15px 0; border-radius: 5px; }
        .stat { display: inline-block; margin-right: 30px; }
        .stat-label { color: #666; font-size: 0.9em; }
        .stat-value { font-size: 1.5em; font-weight: bold; }
        .test-result { background: white; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 5px solid #999; }
        .test-result.pass { border-left-color: #28a745; }
        .test-result.fail { border-left-color: #dc3545; }
        .test-name { font-weight: bold; font-size: 1.1em; }
        .test-status { margin: 10px 0; }
        .pass { color: #28a745; }
        .fail { color: #dc3545; }
        .assertion { margin-left: 20px; padding: 10px; background: #f9f9f9; margin-top: 5px; border-radius: 3px; }
        .assertion.pass { border-left: 3px solid #28a745; }
        .assertion.fail { border-left: 3px solid #dc3545; }
        table { width: 100%; border-collapse: collapse; }
        th, td { padding: 10px; text-align: left; border-bottom: 1px solid #ddd; }
        th { background: #f0f0f0; font-weight: bold; }
    </style>
</head>
<body>
"#,
        );

        html.push_str(&format!(
            r#"<div class="header">
    <h1>Test Report: {}</h1>
    <p>Generated: {}</p>
</div>"#,
            self.suite_name,
            Local::now().format("%Y-%m-%d %H:%M:%S")
        ));

        html.push_str(&format!(
            r#"<div class="summary">
    <h2>Summary</h2>
    <div class="stat">
        <div class="stat-label">Total Tests</div>
        <div class="stat-value">{}</div>
    </div>
    <div class="stat">
        <div class="stat-label">Passed</div>
        <div class="stat-value pass">{}</div>
    </div>
    <div class="stat">
        <div class="stat-label">Failed</div>
        <div class="stat-value fail">{}</div>
    </div>
    <div class="stat">
        <div class="stat-label">Success Rate</div>
        <div class="stat-value">{:.1}%</div>
    </div>
</div>"#,
            self.total_tests(),
            self.passed_tests(),
            self.failed_tests(),
            self.success_rate()
        ));

        html.push_str("<h2>Test Results</h2>");

        for result in &self.results {
            let status_class = if result.passed { "pass" } else { "fail" };
            let status_text = if result.passed { "PASS" } else { "FAIL" };

            html.push_str(&format!(
                r#"<div class="test-result {}">
    <div class="test-name">{}</div>
    <div class="test-status"><span class="{}">{}</span> - {:?}</div>
    <div>Assertions: {}/{}</div>"#,
                status_class,
                result.test_name,
                status_class,
                status_text,
                result.duration,
                result.assertions_passed,
                result.assertions_total
            ));

            if let Some(ref error) = result.error {
                html.push_str(&format!("<div class=\"fail\">Error: {}</div>", error));
            }

            for assertion in &result.assertion_results {
                let assert_class = if assertion.passed { "pass" } else { "fail" };
                html.push_str(&format!(
                    r#"<div class="assertion {}">
        <strong>{}</strong>: {}
        <div style="margin-top: 5px; color: #666;">Expected: {} | Actual: {}</div>
    </div>"#,
                    assert_class,
                    assertion.assertion,
                    if assertion.passed { "✓" } else { "✗" },
                    assertion.expected,
                    assertion.actual
                ));
            }

            html.push_str("</div>");
        }

        html.push_str(
            r#"
</body>
</html>"#,
        );

        html
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_case_creation() {
        let test_case = TestCase {
            name: "test_example".to_string(),
            request: TestRequest {
                method: "GET".to_string(),
                url: "https://example.com".to_string(),
                headers: HashMap::new(),
                body: None,
            },
            assertions: Vec::new(),
            tags: vec!["smoke".to_string()],
            timeout: Duration::from_secs(30),
        };

        assert_eq!(test_case.name, "test_example");
        assert_eq!(test_case.request.method, "GET");
    }

    #[test]
    fn test_test_result_creation() {
        let result = TestResult::new("test_pass".to_string());
        assert!(result.passed);
        assert_eq!(result.assertions_passed, 0);
        assert_eq!(result.assertions_total, 0);
    }

    #[test]
    fn test_test_result_add_assertion() {
        let mut result = TestResult::new("test".to_string());
        let assertion = AssertionResult::new(
            true,
            "status_code".to_string(),
            "200".to_string(),
            "200".to_string(),
            "passed".to_string(),
        );

        result.add_assertion_result(assertion);
        assert_eq!(result.assertions_total, 1);
        assert_eq!(result.assertions_passed, 1);
        assert!(result.passed);
    }

    #[test]
    fn test_test_result_failed_assertion() {
        let mut result = TestResult::new("test".to_string());
        let assertion = AssertionResult::new(
            false,
            "status_code".to_string(),
            "200".to_string(),
            "500".to_string(),
            "failed".to_string(),
        );

        result.add_assertion_result(assertion);
        assert_eq!(result.assertions_total, 1);
        assert_eq!(result.assertions_passed, 0);
        assert!(!result.passed);
    }

    #[test]
    fn test_test_suite_creation() {
        let suite = TestSuite::new("API Tests");
        assert_eq!(suite.name, "API Tests");
        assert_eq!(suite.test_cases.len(), 0);
    }

    #[test]
    fn test_test_suite_add_test() {
        let mut suite = TestSuite::new("API Tests");
        let test_case = TestCase {
            name: "test1".to_string(),
            request: TestRequest {
                method: "GET".to_string(),
                url: "https://example.com".to_string(),
                headers: HashMap::new(),
                body: None,
            },
            assertions: Vec::new(),
            tags: vec![],
            timeout: Duration::from_secs(30),
        };

        suite.add_test(test_case);
        assert_eq!(suite.test_cases.len(), 1);
    }

    #[test]
    fn test_test_report_creation() {
        let report = TestReport::new("Suite".to_string());
        assert_eq!(report.total_tests(), 0);
        assert_eq!(report.passed_tests(), 0);
        assert_eq!(report.failed_tests(), 0);
    }

    #[test]
    fn test_test_report_add_result() {
        let mut report = TestReport::new("Suite".to_string());
        let result = TestResult::new("test1".to_string());
        report.add_result(result);
        assert_eq!(report.total_tests(), 1);
    }

    #[test]
    fn test_test_report_success_rate() {
        let mut report = TestReport::new("Suite".to_string());

        let mut result1 = TestResult::new("test1".to_string());
        result1.passed = true;

        let mut result2 = TestResult::new("test2".to_string());
        result2.passed = false;

        report.add_result(result1);
        report.add_result(result2);

        assert_eq!(report.total_tests(), 2);
        assert_eq!(report.passed_tests(), 1);
        assert_eq!(report.failed_tests(), 1);
        assert!(report.success_rate() > 40.0 && report.success_rate() < 60.0);
    }

    #[test]
    fn test_test_report_summary() {
        let report = TestReport::new("Test Suite".to_string());
        let summary = report.summary();
        assert!(summary.contains("Test Suite"));
        assert!(summary.contains("Total Tests: 0"));
    }

    #[test]
    fn test_html_report_generation() {
        let report = TestReport::new("Suite".to_string());
        let html = report.generate_html_report();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Suite"));
        assert!(html.contains("Test Report"));
    }

    #[test]
    fn test_test_result_summary() {
        let result = TestResult::new("test".to_string());
        let summary = result.summary();
        assert!(summary.contains("0/0"));
    }

    #[test]
    fn test_test_report_assertion_counts() {
        let mut report = TestReport::new("Suite".to_string());

        let mut result = TestResult::new("test1".to_string());
        result.assertions_total = 5;
        result.assertions_passed = 3;

        report.add_result(result);

        assert_eq!(report.total_assertions(), 5);
        assert_eq!(report.passed_assertions(), 3);
    }
}
