use super::auth::Auth;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    url: String,
    method: String,
    headers_map: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body_data: Option<RequestBody>,
    auth_data: Auth,
    request_timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum RequestBody {
    Json(serde_json::Value),
    Form(HashMap<String, String>),
    Text(String),
    Binary(Vec<u8>),
}

impl RequestBody {
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        match self {
            RequestBody::Json(v) => {
                serde_json::to_vec(v).map_err(|e| crate::Error::Http(format!("JSON: {}", e)))
            }
            RequestBody::Form(f) => serde_urlencoded::to_string(f)
                .map(|s| s.into_bytes())
                .map_err(|e| crate::Error::Http(format!("Form: {}", e))),
            RequestBody::Text(t) => Ok(t.as_bytes().to_vec()),
            RequestBody::Binary(b) => Ok(b.clone()),
        }
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            RequestBody::Json(_) => "application/json",
            RequestBody::Form(_) => "application/x-www-form-urlencoded",
            RequestBody::Text(_) => "text/plain",
            RequestBody::Binary(_) => "application/octet-stream",
        }
    }
}

impl RequestBuilder {
    pub fn new(url: impl Into<String>, method: impl Into<String>) -> Self {
        RequestBuilder {
            url: url.into(),
            method: method.into(),
            headers_map: HashMap::new(),
            query_params: HashMap::new(),
            body_data: None,
            auth_data: Auth::None,
            request_timeout: Duration::from_secs(30),
        }
    }

    pub fn get(url: impl Into<String>) -> Self {
        Self::new(url, "GET")
    }
    pub fn post(url: impl Into<String>) -> Self {
        Self::new(url, "POST")
    }
    pub fn put(url: impl Into<String>) -> Self {
        Self::new(url, "PUT")
    }
    pub fn delete(url: impl Into<String>) -> Self {
        Self::new(url, "DELETE")
    }
    pub fn patch(url: impl Into<String>) -> Self {
        Self::new(url, "PATCH")
    }
    pub fn head(url: impl Into<String>) -> Self {
        Self::new(url, "HEAD")
    }
    pub fn options(url: impl Into<String>) -> Self {
        Self::new(url, "OPTIONS")
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers_map.insert(name.into(), value.into());
        self
    }

    pub fn query_param(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(name.into(), value.into());
        self
    }

    pub fn json_body(mut self, value: serde_json::Value) -> Self {
        self.body_data = Some(RequestBody::Json(value));
        self
    }

    pub fn form_body(mut self, form: HashMap<String, String>) -> Self {
        self.body_data = Some(RequestBody::Form(form));
        self
    }

    pub fn basic_auth(mut self, user: impl Into<String>, pass: impl Into<String>) -> Self {
        self.auth_data = Auth::basic(user, pass);
        self
    }

    pub fn bearer_auth(mut self, tok: impl Into<String>) -> Self {
        self.auth_data = Auth::bearer(tok);
        self
    }

    pub fn set_timeout(mut self, dur: Duration) -> Self {
        self.request_timeout = dur;
        self
    }

    pub fn build_url(&self) -> crate::Result<String> {
        let mut url = self.url.clone();
        if !self.query_params.is_empty() {
            let qs = serde_urlencoded::to_string(&self.query_params)
                .map_err(|e| crate::Error::Http(format!("query: {}", e)))?;
            url.push('?');
            url.push_str(&qs);
        }
        Ok(url)
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.url.is_empty() {
            return Err(crate::Error::Http("empty url".into()));
        }
        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err(crate::Error::Http("invalid url scheme".into()));
        }
        self.auth_data.validate()?;
        Ok(())
    }

    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn method(&self) -> &str {
        &self.method
    }
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers_map
    }
    pub fn body(&self) -> &Option<RequestBody> {
        &self.body_data
    }
    pub fn auth(&self) -> &Auth {
        &self.auth_data
    }
    pub fn timeout(&self) -> Duration {
        self.request_timeout
    }
}
