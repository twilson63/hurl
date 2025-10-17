use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub url: String,
    pub headers: HashMap<String, String>,
    pub auth: Option<(String, String)>,
    pub timeout: Option<u64>,
    pub body: Option<String>,
}

impl RequestConfig {
    pub fn parse_url(url: &str) -> Result<String> {
        if url.is_empty() {
            return Err(anyhow!("URL cannot be empty"));
        }

        let url = if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("https://{}", url)
        };

        url::Url::parse(&url)
            .map(|_| url)
            .map_err(|e| anyhow!("Invalid URL: {}", e))
    }

    pub fn parse_headers(header_strings: Vec<String>) -> Result<HashMap<String, String>> {
        let mut headers = HashMap::new();

        for header in header_strings {
            let parts: Vec<&str> = header.splitn(2, ':').collect();
            if parts.len() != 2 {
                return Err(anyhow!(
                    "Invalid header format: '{}'. Expected 'Key: Value'",
                    header
                ));
            }

            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();

            if key.is_empty() {
                return Err(anyhow!("Header key cannot be empty"));
            }

            headers.insert(key, value);
        }

        Ok(headers)
    }

    pub fn parse_auth(auth_str: &str) -> Result<(String, String)> {
        let parts: Vec<&str> = auth_str.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "Invalid auth format: '{}'. Expected 'username:password'",
                auth_str
            ));
        }

        let username = parts[0].to_string();
        let password = parts[1].to_string();

        if username.is_empty() {
            return Err(anyhow!("Username cannot be empty"));
        }

        Ok((username, password))
    }

    pub fn parse_timeout(timeout: Option<u64>) -> Result<Option<std::time::Duration>> {
        match timeout {
            Some(secs) => {
                if secs == 0 {
                    return Err(anyhow!("Timeout must be greater than 0"));
                }
                Ok(Some(std::time::Duration::from_secs(secs)))
            }
            None => Ok(None),
        }
    }

    pub fn new(
        url: &str,
        headers: Vec<String>,
        auth: Option<String>,
        timeout: Option<u64>,
        body: Option<String>,
    ) -> Result<Self> {
        let url = Self::parse_url(url)?;
        let headers = Self::parse_headers(headers)?;
        let auth = auth.as_ref().map(|a| Self::parse_auth(a)).transpose()?;

        Self::parse_timeout(timeout)?;

        Ok(RequestConfig {
            url,
            headers,
            auth,
            timeout,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_with_https() {
        let url = RequestConfig::parse_url("https://api.example.com").unwrap();
        assert_eq!(url, "https://api.example.com");
    }

    #[test]
    fn test_parse_url_with_http() {
        let url = RequestConfig::parse_url("http://api.example.com").unwrap();
        assert_eq!(url, "http://api.example.com");
    }

    #[test]
    fn test_parse_url_without_scheme() {
        let url = RequestConfig::parse_url("api.example.com").unwrap();
        assert_eq!(url, "https://api.example.com");
    }

    #[test]
    fn test_parse_url_empty() {
        let result = RequestConfig::parse_url("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_url_invalid() {
        let result = RequestConfig::parse_url("not a url");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_headers_valid() {
        let headers = RequestConfig::parse_headers(vec![
            "Content-Type: application/json".to_string(),
            "Accept: application/json".to_string(),
        ])
        .unwrap();

        assert_eq!(headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(headers.get("Accept").unwrap(), "application/json");
    }

    #[test]
    fn test_parse_headers_invalid_format() {
        let result = RequestConfig::parse_headers(vec!["InvalidHeader".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_headers_empty_key() {
        let result = RequestConfig::parse_headers(vec![": value".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_auth_valid() {
        let (user, pass) = RequestConfig::parse_auth("user:password").unwrap();
        assert_eq!(user, "user");
        assert_eq!(pass, "password");
    }

    #[test]
    fn test_parse_auth_invalid_format() {
        let result = RequestConfig::parse_auth("nopassword");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_auth_empty_username() {
        let result = RequestConfig::parse_auth(":password");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_timeout_valid() {
        let timeout = RequestConfig::parse_timeout(Some(30)).unwrap();
        assert_eq!(timeout.unwrap().as_secs(), 30);
    }

    #[test]
    fn test_parse_timeout_zero() {
        let result = RequestConfig::parse_timeout(Some(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_timeout_none() {
        let timeout = RequestConfig::parse_timeout(None).unwrap();
        assert!(timeout.is_none());
    }
}
