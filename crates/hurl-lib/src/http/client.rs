use super::compression::CompressionConfig;
use super::request::RequestBuilder;
use super::response::HttpResponse;
use super::security::{ProxyConfig, TlsConfig};
use reqwest::cookie::Jar;
use reqwest::Client as ReqClient;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct HttpClient {
    client: Arc<ReqClient>,
    config: Arc<ClientConfig>,
}

pub struct ClientConfig {
    pub max_connections: usize,
    pub default_timeout_secs: u64,
    pub enable_cookies: bool,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub max_redirects: u32,
    pub tls_config: TlsConfig,
    pub proxy_config: ProxyConfig,
    pub compression_config: CompressionConfig,
}

impl Default for ClientConfig {
    fn default() -> Self {
        ClientConfig {
            max_connections: 32,
            default_timeout_secs: 30,
            enable_cookies: true,
            max_retries: 3,
            retry_delay_ms: 100,
            max_redirects: 5,
            tls_config: TlsConfig::new(),
            proxy_config: ProxyConfig::new(),
            compression_config: CompressionConfig::new(),
        }
    }
}

impl HttpClient {
    pub fn new() -> crate::Result<Self> {
        Self::with_config(ClientConfig::default())
    }

    pub fn with_config(config: ClientConfig) -> crate::Result<Self> {
        config.tls_config.validate()?;
        config.proxy_config.validate()?;

        let jar = Arc::new(Jar::default());
        let mut builder = ReqClient::builder()
            .pool_max_idle_per_host(config.max_connections)
            .cookie_provider(jar);

        builder = match &config.proxy_config.proxy {
            Some(super::security::ProxyType::Http { url }) => builder.proxy(
                reqwest::Proxy::http(url)
                    .map_err(|e| crate::Error::Http(format!("Proxy config failed: {}", e)))?,
            ),
            Some(super::security::ProxyType::Https { url }) => builder.proxy(
                reqwest::Proxy::https(url)
                    .map_err(|e| crate::Error::Http(format!("Proxy config failed: {}", e)))?,
            ),
            Some(super::security::ProxyType::Socks5 { url }) => builder.proxy(
                reqwest::Proxy::all(url)
                    .map_err(|e| crate::Error::Http(format!("Proxy config failed: {}", e)))?,
            ),
            None => builder,
        };

        let client = builder
            .build()
            .map_err(|e| crate::Error::Http(format!("HTTP client failed: {}", e)))?;
        Ok(HttpClient {
            client: Arc::new(client),
            config: Arc::new(config),
        })
    }

    pub async fn execute(&self, builder: RequestBuilder) -> crate::Result<HttpResponse> {
        self.execute_with_redirects(builder, 0).await
    }

    async fn execute_with_redirects(
        &self,
        builder: RequestBuilder,
        redirect_count: u32,
    ) -> crate::Result<HttpResponse> {
        if redirect_count > self.config.max_redirects {
            return Err(crate::Error::Http(format!(
                "Maximum redirects exceeded: {}",
                self.config.max_redirects
            )));
        }

        builder.validate()?;
        let url = builder.build_url()?;
        let start = Instant::now();

        let mut request = match builder.method() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            "PATCH" => self.client.patch(&url),
            "HEAD" => self.client.head(&url),
            "OPTIONS" => self.client.request(reqwest::Method::OPTIONS, &url),
            m => return Err(crate::Error::Http(format!("Unknown method: {}", m))),
        };

        let timeout_dur = builder.timeout();
        request = request.timeout(timeout_dur);

        if self.config.compression_config.enabled {
            request = request.header(
                "Accept-Encoding",
                self.config.compression_config.accept_encoding_header(),
            );
        }

        for (n, v) in builder.headers() {
            request = request.header(n, v);
        }

        if let Some(auth_h) = builder.auth().header_value() {
            request = request.header("Authorization", auth_h);
        }

        if let Some(body) = builder.body() {
            let ct = body.content_type();
            request = request.header("Content-Type", ct);
            let bytes = body.to_bytes()?;
            request = request.body(bytes);
        }

        let resp = request
            .send()
            .await
            .map_err(|e| crate::Error::Http(format!("Request failed: {}", e)))?;

        let status = resp.status().as_u16();
        let headers = resp
            .headers()
            .iter()
            .map(|(n, v)| (n.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let body_bytes = resp
            .bytes()
            .await
            .map_err(|e| crate::Error::Http(format!("Body read failed: {}", e)))?;

        let body_text = String::from_utf8_lossy(&body_bytes).to_string();

        let dur = start.elapsed();
        let http_resp = HttpResponse::new(status, headers, body_text).with_duration(dur);

        Ok(http_resp)
    }

    pub async fn get(&self, url: impl Into<String>) -> crate::Result<HttpResponse> {
        self.execute(RequestBuilder::get(url)).await
    }

    pub async fn post_json(
        &self,
        url: impl Into<String>,
        json: serde_json::Value,
    ) -> crate::Result<HttpResponse> {
        self.execute(RequestBuilder::post(url).json_body(json))
            .await
    }

    pub async fn post_form(
        &self,
        url: impl Into<String>,
        form: HashMap<String, String>,
    ) -> crate::Result<HttpResponse> {
        self.execute(RequestBuilder::post(url).form_body(form))
            .await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}
