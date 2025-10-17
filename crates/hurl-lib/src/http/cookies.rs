use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSite,
    pub expires: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Cookie {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Cookie {
            name: name.into(),
            value: value.into(),
            domain: None,
            path: Some("/".to_string()),
            secure: false,
            http_only: false,
            same_site: SameSite::Lax,
            expires: None,
        }
    }

    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn secure(mut self) -> Self {
        self.secure = true;
        self
    }

    pub fn http_only(mut self) -> Self {
        self.http_only = true;
        self
    }

    pub fn with_same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = same_site;
        self
    }

    pub fn with_expires(mut self, timestamp: u64) -> Self {
        self.expires = Some(timestamp);
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(exp) = self.expires {
            if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
                return exp < duration.as_secs();
            }
        }
        false
    }

    pub fn matches_domain(&self, request_domain: &str) -> bool {
        match &self.domain {
            Some(domain) => {
                if domain.starts_with('.') {
                    request_domain.ends_with(domain) || request_domain.ends_with(&domain[1..])
                } else {
                    request_domain == domain
                }
            }
            None => true,
        }
    }

    pub fn matches_path(&self, request_path: &str) -> bool {
        match &self.path {
            Some(path) => request_path.starts_with(path),
            None => true,
        }
    }

    pub fn to_header_value(&self) -> String {
        format!("{}={}", self.name, self.value)
    }

    pub fn to_set_cookie_header(&self) -> String {
        let mut header = format!("{}={}", self.name, self.value);
        if let Some(domain) = &self.domain {
            header.push_str(&format!("; Domain={}", domain));
        }
        if let Some(path) = &self.path {
            header.push_str(&format!("; Path={}", path));
        }
        if let Some(expires) = self.expires {
            header.push_str(&format!("; Expires={}", expires));
        }
        if self.secure {
            header.push_str("; Secure");
        }
        if self.http_only {
            header.push_str("; HttpOnly");
        }
        let same_site_str = match self.same_site {
            SameSite::Strict => "Strict",
            SameSite::Lax => "Lax",
            SameSite::None => "None",
        };
        header.push_str(&format!("; SameSite={}", same_site_str));
        header
    }
}

#[derive(Debug, Clone)]
pub struct CookieJar {
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    pub fn new() -> Self {
        CookieJar {
            cookies: HashMap::new(),
        }
    }

    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }

    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<Cookie> {
        self.cookies.remove(name)
    }

    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    pub fn get_for_url(&self, domain: &str, path: &str) -> Vec<&Cookie> {
        self.cookies
            .values()
            .filter(|c| !c.is_expired() && c.matches_domain(domain) && c.matches_path(path))
            .collect()
    }

    pub fn get_cookie_header(&self, domain: &str, path: &str) -> Option<String> {
        let matching = self.get_for_url(domain, path);
        if matching.is_empty() {
            None
        } else {
            let cookie_str = matching
                .iter()
                .map(|c| c.to_header_value())
                .collect::<Vec<_>>()
                .join("; ");
            Some(cookie_str)
        }
    }

    pub fn len(&self) -> usize {
        self.cookies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty()
    }

    pub fn all(&self) -> Vec<&Cookie> {
        self.cookies.values().collect()
    }

    pub fn remove_expired(&mut self) {
        self.cookies.retain(|_, c| !c.is_expired());
    }
}

impl Default for CookieJar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct CookiePersistence {
    file_path: std::path::PathBuf,
}

impl CookiePersistence {
    pub fn new(file_path: impl Into<std::path::PathBuf>) -> Self {
        CookiePersistence {
            file_path: file_path.into(),
        }
    }

    pub fn save(&self, jar: &CookieJar) -> crate::Result<()> {
        let cookies = jar
            .all()
            .iter()
            .map(|c| {
                serde_json::json!({
                    "name": c.name,
                    "value": c.value,
                    "domain": c.domain,
                    "path": c.path,
                    "secure": c.secure,
                    "http_only": c.http_only,
                    "same_site": match c.same_site {
                        SameSite::Strict => "Strict",
                        SameSite::Lax => "Lax",
                        SameSite::None => "None",
                    },
                    "expires": c.expires,
                })
            })
            .collect::<Vec<_>>();

        let json = serde_json::json!(cookies);
        let content = serde_json::to_string_pretty(&json)
            .map_err(|e| crate::Error::Http(format!("Failed to serialize cookies: {}", e)))?;
        std::fs::write(&self.file_path, content).map_err(crate::Error::Io)?;
        Ok(())
    }

    pub fn load(&self) -> crate::Result<CookieJar> {
        if !self.file_path.exists() {
            return Ok(CookieJar::new());
        }

        let content = std::fs::read_to_string(&self.file_path).map_err(crate::Error::Io)?;
        let json: Vec<serde_json::Value> = serde_json::from_str(&content)
            .map_err(|e| crate::Error::Http(format!("Failed to deserialize cookies: {}", e)))?;

        let mut jar = CookieJar::new();
        for cookie_json in json {
            let name = cookie_json["name"]
                .as_str()
                .ok_or_else(|| crate::Error::Http("Missing cookie name".into()))?
                .to_string();
            let value = cookie_json["value"]
                .as_str()
                .ok_or_else(|| crate::Error::Http("Missing cookie value".into()))?
                .to_string();

            let mut cookie = Cookie::new(name, value);
            if let Some(domain) = cookie_json["domain"].as_str() {
                cookie = cookie.with_domain(domain);
            }
            if let Some(path) = cookie_json["path"].as_str() {
                cookie = cookie.with_path(path);
            }
            if cookie_json["secure"].as_bool().unwrap_or(false) {
                cookie = cookie.secure();
            }
            if cookie_json["http_only"].as_bool().unwrap_or(false) {
                cookie = cookie.http_only();
            }
            let same_site_str = cookie_json["same_site"].as_str().unwrap_or("Lax");
            let same_site = match same_site_str {
                "Strict" => SameSite::Strict,
                "None" => SameSite::None,
                _ => SameSite::Lax,
            };
            cookie = cookie.with_same_site(same_site);
            if let Some(expires) = cookie_json["expires"].as_u64() {
                cookie = cookie.with_expires(expires);
            }

            jar.add(cookie);
        }

        jar.remove_expired();
        Ok(jar)
    }
}
