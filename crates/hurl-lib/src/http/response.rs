use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::time::Duration;

fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u128(duration.as_millis())
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u128::deserialize(deserializer)?;
    Ok(Duration::from_millis(millis as u64))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    #[serde(
        serialize_with = "serialize_duration",
        deserialize_with = "deserialize_duration"
    )]
    pub duration: Duration,
}

impl HttpResponse {
    pub fn new(status: u16, headers: HashMap<String, String>, body: String) -> Self {
        HttpResponse {
            status,
            headers,
            body,
            duration: Duration::from_secs(0),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    pub fn is_client_error(&self) -> bool {
        self.status >= 400 && self.status < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status >= 500 && self.status < 600
    }

    pub fn parse_json<T: serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
        serde_json::from_str(&self.body).map_err(|e| crate::Error::Http(format!("JSON: {}", e)))
    }

    pub fn try_parse_json<T: serde::de::DeserializeOwned>(&self) -> Option<T> {
        serde_json::from_str(&self.body).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::auth::Auth;
    use crate::http::client::{ClientConfig, HttpClient};
    use crate::http::request::{RequestBody, RequestBuilder};

    #[test]
    fn test_basic_auth_encode() {
        let a = Auth::basic("user", "pass");
        let h = a.header_value().unwrap();
        assert!(h.starts_with("Basic "));
    }

    #[test]
    fn test_bearer_auth() {
        let a = Auth::bearer("token123");
        let h = a.header_value().unwrap();
        assert_eq!(h, "Bearer token123");
    }

    #[test]
    fn test_none_auth() {
        let a = Auth::None;
        assert!(a.header_value().is_none());
    }

    #[test]
    fn test_basic_validate() {
        let a = Auth::basic("u", "p");
        assert!(a.validate().is_ok());

        let empty_user = Auth::Basic {
            username: "".to_string(),
            password: "p".to_string(),
        };
        assert!(empty_user.validate().is_err());
    }

    #[test]
    fn test_bearer_validate() {
        let a = Auth::bearer("token");
        assert!(a.validate().is_ok());

        let empty = Auth::Bearer {
            token: "".to_string(),
        };
        assert!(empty.validate().is_err());
    }

    #[test]
    fn test_response_creation() {
        let mut h = std::collections::HashMap::new();
        h.insert("content-type".to_string(), "application/json".to_string());
        let r = HttpResponse::new(200, h, "{}".to_string());
        assert_eq!(r.status, 200);
        assert_eq!(r.body, "{}");
    }

    #[test]
    fn test_response_header() {
        let mut h = std::collections::HashMap::new();
        h.insert("Content-Type".to_string(), "application/json".to_string());
        let r = HttpResponse::new(200, h, "".to_string());
        assert_eq!(r.header("content-type"), Some("application/json"));
    }

    #[test]
    fn test_response_success() {
        let r = HttpResponse::new(200, std::collections::HashMap::new(), "".to_string());
        assert!(r.is_success());
        assert!(!r.is_client_error());
        assert!(!r.is_server_error());
    }

    #[test]
    fn test_response_client_error() {
        let r = HttpResponse::new(404, std::collections::HashMap::new(), "".to_string());
        assert!(!r.is_success());
        assert!(r.is_client_error());
    }

    #[test]
    fn test_response_server_error() {
        let r = HttpResponse::new(500, std::collections::HashMap::new(), "".to_string());
        assert!(!r.is_success());
        assert!(r.is_server_error());
    }

    #[test]
    fn test_response_json_parse() {
        let r = HttpResponse::new(
            200,
            std::collections::HashMap::new(),
            r#"{"key":"value"}"#.to_string(),
        );
        let v: serde_json::Value = r.parse_json().unwrap();
        assert_eq!(v["key"], "value");
    }

    #[test]
    fn test_request_builder() {
        let rb = RequestBuilder::get("https://example.com");
        assert_eq!(rb.method(), "GET");
        assert_eq!(rb.url(), "https://example.com");
    }

    #[test]
    fn test_all_methods() {
        assert_eq!(RequestBuilder::get("h").method(), "GET");
        assert_eq!(RequestBuilder::post("h").method(), "POST");
        assert_eq!(RequestBuilder::put("h").method(), "PUT");
        assert_eq!(RequestBuilder::delete("h").method(), "DELETE");
        assert_eq!(RequestBuilder::patch("h").method(), "PATCH");
        assert_eq!(RequestBuilder::head("h").method(), "HEAD");
        assert_eq!(RequestBuilder::options("h").method(), "OPTIONS");
    }

    #[test]
    fn test_headers() {
        let rb = RequestBuilder::get("h").header("X-Custom", "value");
        assert_eq!(rb.headers().len(), 1);
    }

    #[test]
    fn test_query_params() {
        let rb = RequestBuilder::get("h").query_param("key", "value");
        let url = rb.build_url().unwrap();
        assert!(url.contains("?"));
        assert!(url.contains("key=value"));
    }

    #[test]
    fn test_json_body() {
        let j = serde_json::json!({"test": "value"});
        let rb = RequestBuilder::post("h").json_body(j);
        assert!(rb.body().is_some());
    }

    #[test]
    fn test_form_body() {
        let mut f = std::collections::HashMap::new();
        f.insert("key".to_string(), "value".to_string());
        let rb = RequestBuilder::post("h").form_body(f);
        assert!(rb.body().is_some());
    }

    #[test]
    fn test_basic_auth_builder() {
        let rb = RequestBuilder::get("h").basic_auth("user", "pass");
        if let Auth::Basic { .. } = rb.auth() {
            // ok
        } else {
            panic!("expected Basic auth");
        }
    }

    #[test]
    fn test_bearer_auth_builder() {
        let rb = RequestBuilder::get("h").bearer_auth("token");
        if let Auth::Bearer { .. } = rb.auth() {
            // ok
        } else {
            panic!("expected Bearer auth");
        }
    }

    #[test]
    fn test_timeout() {
        let rb = RequestBuilder::get("h").set_timeout(Duration::from_secs(60));
        assert_eq!(rb.timeout(), Duration::from_secs(60));
    }

    #[test]
    fn test_validate_empty_url() {
        let rb = RequestBuilder::get("");
        assert!(rb.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_scheme() {
        let rb = RequestBuilder::get("ftp://example.com");
        assert!(rb.validate().is_err());
    }

    #[test]
    fn test_validate_valid() {
        let rb = RequestBuilder::get("https://example.com");
        assert!(rb.validate().is_ok());
        let rb2 = RequestBuilder::get("http://example.com");
        assert!(rb2.validate().is_ok());
    }

    #[test]
    fn test_request_body_json() {
        let b = RequestBody::Json(serde_json::json!({}));
        assert_eq!(b.content_type(), "application/json");
    }

    #[test]
    fn test_request_body_form() {
        let b = RequestBody::Form(std::collections::HashMap::new());
        assert_eq!(b.content_type(), "application/x-www-form-urlencoded");
    }

    #[test]
    fn test_request_body_text() {
        let b = RequestBody::Text("text".to_string());
        assert_eq!(b.content_type(), "text/plain");
    }

    #[test]
    fn test_client_creation() {
        assert!(HttpClient::new().is_ok());
    }

    #[test]
    fn test_client_config() {
        let c = ClientConfig::default();
        assert_eq!(c.max_connections, 32);
        assert_eq!(c.default_timeout_secs, 30);
        assert!(c.enable_cookies);
    }
}
