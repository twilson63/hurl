use base64::{engine::general_purpose::STANDARD, Engine};
use md5;

#[derive(Debug, Clone)]
#[derive(Default)]
pub enum Auth {
    #[default]
    None,
    Basic {
        username: String,
        password: String,
    },
    Bearer {
        token: String,
    },
    Digest {
        username: String,
        password: String,
        realm: Option<String>,
        nonce: Option<String>,
        uri: Option<String>,
        qop: Option<String>,
        nc: u32,
        cnonce: Option<String>,
    },
    OAuth2 {
        token: String,
        token_type: String,
        expires_in: Option<u64>,
        refresh_token: Option<String>,
    },
    Kerberos {
        principal: String,
    },
}

impl Auth {
    pub fn basic(u: impl Into<String>, p: impl Into<String>) -> Self {
        Auth::Basic {
            username: u.into(),
            password: p.into(),
        }
    }

    pub fn bearer(t: impl Into<String>) -> Self {
        Auth::Bearer { token: t.into() }
    }

    pub fn digest(u: impl Into<String>, p: impl Into<String>) -> Self {
        Auth::Digest {
            username: u.into(),
            password: p.into(),
            realm: None,
            nonce: None,
            uri: None,
            qop: None,
            nc: 0,
            cnonce: None,
        }
    }

    pub fn oauth2(token: impl Into<String>) -> Self {
        Auth::OAuth2 {
            token: token.into(),
            token_type: "Bearer".to_string(),
            expires_in: None,
            refresh_token: None,
        }
    }

    pub fn oauth2_with_refresh(
        token: impl Into<String>,
        refresh_token: impl Into<String>,
        expires_in: u64,
    ) -> Self {
        Auth::OAuth2 {
            token: token.into(),
            token_type: "Bearer".to_string(),
            expires_in: Some(expires_in),
            refresh_token: Some(refresh_token.into()),
        }
    }

    pub fn kerberos(principal: impl Into<String>) -> Self {
        Auth::Kerberos {
            principal: principal.into(),
        }
    }

    pub fn header_value(&self) -> Option<String> {
        match self {
            Auth::None => None,
            Auth::Basic { username, password } => {
                let cred = format!("{}:{}", username, password);
                Some(format!("Basic {}", STANDARD.encode(&cred)))
            }
            Auth::Bearer { token } => Some(format!("Bearer {}", token)),
            Auth::Digest {
                username,
                password,
                realm,
                nonce,
                uri,
                qop,
                nc,
                cnonce,
            } => {
                let mut digest_header = format!("Digest username=\"{}\"", username);
                if let Some(r) = realm {
                    digest_header.push_str(&format!(", realm=\"{}\"", r));
                }
                if let Some(n) = nonce {
                    digest_header.push_str(&format!(", nonce=\"{}\"", n));
                    let response_hash = Self::compute_digest_response(
                        username,
                        password,
                        realm.as_deref().unwrap_or(""),
                        n,
                        uri.as_deref().unwrap_or("/"),
                        "GET",
                        qop.as_deref(),
                        *nc,
                        cnonce.as_deref(),
                    );
                    digest_header.push_str(&format!(", response=\"{}\"", response_hash));
                }
                if let Some(u) = uri {
                    digest_header.push_str(&format!(", uri=\"{}\"", u));
                }
                if let Some(q) = qop {
                    digest_header.push_str(&format!(", qop={}", q));
                    digest_header.push_str(&format!(", nc={:08x}", nc));
                    if let Some(c) = cnonce {
                        digest_header.push_str(&format!(", cnonce=\"{}\"", c));
                    }
                }
                Some(digest_header)
            }
            Auth::OAuth2 {
                token, token_type, ..
            } => Some(format!("{} {}", token_type, token)),
            Auth::Kerberos { .. } => Some("Negotiate".to_string()),
        }
    }

    fn compute_digest_response(
        username: &str,
        password: &str,
        realm: &str,
        nonce: &str,
        uri: &str,
        method: &str,
        qop: Option<&str>,
        nc: u32,
        cnonce: Option<&str>,
    ) -> String {
        let ha1 = format!("{}:{}:{}", username, realm, password);
        let ha1_hash = format!("{:x}", md5::compute(ha1.as_bytes()));

        let ha2 = format!("{}:{}", method, uri);
        let ha2_hash = format!("{:x}", md5::compute(ha2.as_bytes()));

        let response = if let Some(q) = qop {
            let cnonce_val = cnonce.unwrap_or("0");
            format!(
                "{}:{}:{:08x}:{}:{}:{}",
                ha1_hash, nonce, nc, cnonce_val, q, ha2_hash
            )
        } else {
            format!("{}:{}:{}", ha1_hash, nonce, ha2_hash)
        };

        format!("{:x}", md5::compute(response.as_bytes()))
    }

    pub fn validate(&self) -> crate::Result<()> {
        match self {
            Auth::None => Ok(()),
            Auth::Basic { username, password } => {
                if username.is_empty() || password.is_empty() {
                    Err(crate::Error::Http("credentials required".into()))
                } else {
                    Ok(())
                }
            }
            Auth::Bearer { token } => {
                if token.is_empty() {
                    Err(crate::Error::Http("token required".into()))
                } else {
                    Ok(())
                }
            }
            Auth::Digest {
                username, password, ..
            } => {
                if username.is_empty() || password.is_empty() {
                    Err(crate::Error::Http("digest credentials required".into()))
                } else {
                    Ok(())
                }
            }
            Auth::OAuth2 { token, .. } => {
                if token.is_empty() {
                    Err(crate::Error::Http("oauth2 token required".into()))
                } else {
                    Ok(())
                }
            }
            Auth::Kerberos { principal } => {
                if principal.is_empty() {
                    Err(crate::Error::Http("kerberos principal required".into()))
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Auth::OAuth2 { expires_in, .. } = self {
            expires_in == &Some(0)
        } else {
            false
        }
    }

    pub fn get_refresh_token(&self) -> Option<&str> {
        if let Auth::OAuth2 { refresh_token, .. } = self {
            refresh_token.as_deref()
        } else {
            None
        }
    }
}

