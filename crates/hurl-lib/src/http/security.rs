use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum CertificateValidation {
    Strict,
    Permissive,
    Custom { ca_bundle: PathBuf },
}

#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub validation: CertificateValidation,
    pub cert_pinning: Option<Vec<String>>,
    pub client_cert: Option<PathBuf>,
    pub client_key: Option<PathBuf>,
    pub min_tls_version: TlsVersion,
    pub ciphers: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    TLS1_0,
    TLS1_1,
    TLS1_2,
    TLS1_3,
}

impl TlsConfig {
    pub fn new() -> Self {
        TlsConfig {
            validation: CertificateValidation::Strict,
            cert_pinning: None,
            client_cert: None,
            client_key: None,
            min_tls_version: TlsVersion::TLS1_2,
            ciphers: vec![],
        }
    }

    pub fn strict() -> Self {
        Self::new()
    }

    pub fn permissive() -> Self {
        TlsConfig {
            validation: CertificateValidation::Permissive,
            ..Self::new()
        }
    }

    pub fn with_ca_bundle(ca_bundle: PathBuf) -> Self {
        TlsConfig {
            validation: CertificateValidation::Custom { ca_bundle },
            ..Self::new()
        }
    }

    pub fn with_cert_pinning(mut self, pins: Vec<String>) -> Self {
        self.cert_pinning = Some(pins);
        self
    }

    pub fn with_client_cert(mut self, cert: PathBuf, key: PathBuf) -> Self {
        self.client_cert = Some(cert);
        self.client_key = Some(key);
        self
    }

    pub fn with_min_tls_version(mut self, version: TlsVersion) -> Self {
        self.min_tls_version = version;
        self
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.client_cert.is_some() && self.client_key.is_none() {
            return Err(crate::Error::Config(
                "client certificate requires key".into(),
            ));
        }
        if self.client_cert.is_none() && self.client_key.is_some() {
            return Err(crate::Error::Config(
                "client key requires certificate".into(),
            ));
        }
        Ok(())
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ProxyType {
    Http { url: String },
    Https { url: String },
    Socks5 { url: String },
}

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub proxy: Option<ProxyType>,
    pub no_proxy: Vec<String>,
    pub auth: Option<ProxyAuth>,
}

#[derive(Debug, Clone)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

impl ProxyConfig {
    pub fn new() -> Self {
        ProxyConfig {
            proxy: None,
            no_proxy: vec![],
            auth: None,
        }
    }

    pub fn http(url: impl Into<String>) -> Self {
        ProxyConfig {
            proxy: Some(ProxyType::Http { url: url.into() }),
            ..Self::new()
        }
    }

    pub fn https(url: impl Into<String>) -> Self {
        ProxyConfig {
            proxy: Some(ProxyType::Https { url: url.into() }),
            ..Self::new()
        }
    }

    pub fn socks5(url: impl Into<String>) -> Self {
        ProxyConfig {
            proxy: Some(ProxyType::Socks5 { url: url.into() }),
            ..Self::new()
        }
    }

    pub fn with_no_proxy(mut self, domains: Vec<String>) -> Self {
        self.no_proxy = domains;
        self
    }

    pub fn with_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.auth = Some(ProxyAuth {
            username: username.into(),
            password: password.into(),
        });
        self
    }

    pub fn should_bypass(&self, host: &str) -> bool {
        self.no_proxy.iter().any(|np| {
            if np.starts_with('.') {
                host.ends_with(np) || host.ends_with(&np[1..])
            } else {
                host == np
            }
        })
    }

    pub fn validate(&self) -> crate::Result<()> {
        if let Some(
            ProxyType::Http { url } | ProxyType::Https { url } | ProxyType::Socks5 { url },
        ) = &self.proxy
        {
            if url.is_empty() {
                return Err(crate::Error::Config("proxy URL cannot be empty".into()));
            }
        }
        Ok(())
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SecureCredentialStore {
    credentials: std::collections::HashMap<String, String>,
}

impl SecureCredentialStore {
    pub fn new() -> Self {
        SecureCredentialStore {
            credentials: std::collections::HashMap::new(),
        }
    }

    pub fn store(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.credentials.insert(key.into(), value.into());
    }

    pub fn retrieve(&self, key: &str) -> Option<String> {
        self.credentials.get(key).cloned()
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.credentials.remove(key)
    }

    pub fn clear(&mut self) {
        self.credentials.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.credentials.is_empty()
    }
}

impl Default for SecureCredentialStore {
    fn default() -> Self {
        Self::new()
    }
}
