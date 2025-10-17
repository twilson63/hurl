#[cfg(test)]
mod tests {
    use crate::http::auth::Auth;
    use crate::http::compression::{CompressionCodec, CompressionConfig};
    use crate::http::cookies::{Cookie, CookieJar, SameSite};
    use crate::http::security::{
        CertificateValidation, ProxyConfig, ProxyType, SecureCredentialStore, TlsConfig, TlsVersion,
    };

    mod auth_tests {
        use super::*;

        #[test]
        fn test_basic_auth_creation() {
            let auth = Auth::basic("user", "pass");
            match auth {
                Auth::Basic { username, password } => {
                    assert_eq!(username, "user");
                    assert_eq!(password, "pass");
                }
                _ => panic!("Expected Basic auth"),
            }
        }

        #[test]
        fn test_basic_auth_header() {
            let auth = Auth::basic("user", "pass");
            let header = auth.header_value().unwrap();
            assert!(header.starts_with("Basic "));
        }

        #[test]
        fn test_bearer_auth_creation() {
            let auth = Auth::bearer("token123");
            match auth {
                Auth::Bearer { token } => {
                    assert_eq!(token, "token123");
                }
                _ => panic!("Expected Bearer auth"),
            }
        }

        #[test]
        fn test_bearer_auth_header() {
            let auth = Auth::bearer("mytoken");
            let header = auth.header_value().unwrap();
            assert_eq!(header, "Bearer mytoken");
        }

        #[test]
        fn test_digest_auth_creation() {
            let auth = Auth::digest("user", "pass");
            match auth {
                Auth::Digest {
                    username, password, ..
                } => {
                    assert_eq!(username, "user");
                    assert_eq!(password, "pass");
                }
                _ => panic!("Expected Digest auth"),
            }
        }

        #[test]
        fn test_digest_auth_header() {
            let auth = Auth::digest("user", "pass");
            let header = auth.header_value();
            assert!(header.is_some());
            let h = header.unwrap();
            assert!(h.starts_with("Digest "));
        }

        #[test]
        fn test_oauth2_auth_creation() {
            let auth = Auth::oauth2("access_token");
            match auth {
                Auth::OAuth2 {
                    token, token_type, ..
                } => {
                    assert_eq!(token, "access_token");
                    assert_eq!(token_type, "Bearer");
                }
                _ => panic!("Expected OAuth2 auth"),
            }
        }

        #[test]
        fn test_oauth2_with_refresh() {
            let auth = Auth::oauth2_with_refresh("token", "refresh", 3600);
            match auth {
                Auth::OAuth2 {
                    token,
                    refresh_token,
                    expires_in,
                    ..
                } => {
                    assert_eq!(token, "token");
                    assert_eq!(refresh_token, Some("refresh".to_string()));
                    assert_eq!(expires_in, Some(3600));
                }
                _ => panic!("Expected OAuth2 auth"),
            }
        }

        #[test]
        fn test_oauth2_header() {
            let auth = Auth::oauth2("mytoken");
            let header = auth.header_value().unwrap();
            assert_eq!(header, "Bearer mytoken");
        }

        #[test]
        fn test_kerberos_auth_creation() {
            let auth = Auth::kerberos("user@REALM");
            match auth {
                Auth::Kerberos { principal } => {
                    assert_eq!(principal, "user@REALM");
                }
                _ => panic!("Expected Kerberos auth"),
            }
        }

        #[test]
        fn test_kerberos_header() {
            let auth = Auth::kerberos("user@REALM");
            let header = auth.header_value().unwrap();
            assert_eq!(header, "Negotiate");
        }

        #[test]
        fn test_none_auth() {
            let auth = Auth::None;
            assert!(auth.header_value().is_none());
        }

        #[test]
        fn test_basic_auth_validate() {
            let auth = Auth::basic("user", "pass");
            assert!(auth.validate().is_ok());
        }

        #[test]
        fn test_basic_auth_validate_empty_user() {
            let auth = Auth::Basic {
                username: "".to_string(),
                password: "pass".to_string(),
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_basic_auth_validate_empty_password() {
            let auth = Auth::Basic {
                username: "user".to_string(),
                password: "".to_string(),
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_bearer_validate() {
            let auth = Auth::bearer("token");
            assert!(auth.validate().is_ok());
        }

        #[test]
        fn test_bearer_validate_empty() {
            let auth = Auth::Bearer {
                token: "".to_string(),
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_digest_validate() {
            let auth = Auth::digest("user", "pass");
            assert!(auth.validate().is_ok());
        }

        #[test]
        fn test_oauth2_validate() {
            let auth = Auth::oauth2("token");
            assert!(auth.validate().is_ok());
        }

        #[test]
        fn test_oauth2_validate_empty() {
            let auth = Auth::OAuth2 {
                token: "".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: None,
                refresh_token: None,
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_oauth2_is_expired() {
            let auth = Auth::OAuth2 {
                token: "token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: Some(0),
                refresh_token: None,
            };
            assert!(auth.is_expired());
        }

        #[test]
        fn test_oauth2_not_expired() {
            let auth = Auth::oauth2_with_refresh("token", "refresh", 3600);
            assert!(!auth.is_expired());
        }

        #[test]
        fn test_get_refresh_token() {
            let auth = Auth::oauth2_with_refresh("token", "refresh_token", 3600);
            assert_eq!(auth.get_refresh_token(), Some("refresh_token"));
        }

        #[test]
        fn test_get_refresh_token_none() {
            let auth = Auth::oauth2("token");
            assert_eq!(auth.get_refresh_token(), None);
        }

        #[test]
        fn test_auth_default() {
            let auth = Auth::default();
            assert!(matches!(auth, Auth::None));
        }
    }

    mod security_tests {
        use super::*;

        #[test]
        fn test_tls_config_new() {
            let config = TlsConfig::new();
            assert_eq!(config.min_tls_version, TlsVersion::TLS1_2);
            assert!(config.cert_pinning.is_none());
        }

        #[test]
        fn test_tls_config_strict() {
            let config = TlsConfig::strict();
            assert!(matches!(config.validation, CertificateValidation::Strict));
        }

        #[test]
        fn test_tls_config_permissive() {
            let config = TlsConfig::permissive();
            assert!(matches!(
                config.validation,
                CertificateValidation::Permissive
            ));
        }

        #[test]
        fn test_tls_config_with_ca_bundle() {
            let path = std::path::PathBuf::from("/path/to/ca.pem");
            let config = TlsConfig::with_ca_bundle(path.clone());
            assert!(matches!(
                config.validation,
                CertificateValidation::Custom { .. }
            ));
        }

        #[test]
        fn test_tls_config_with_cert_pinning() {
            let config =
                TlsConfig::new().with_cert_pinning(vec!["pin1".to_string(), "pin2".to_string()]);
            assert_eq!(config.cert_pinning.unwrap().len(), 2);
        }

        #[test]
        fn test_tls_config_with_min_tls_version() {
            let config = TlsConfig::new().with_min_tls_version(TlsVersion::TLS1_3);
            assert_eq!(config.min_tls_version, TlsVersion::TLS1_3);
        }

        #[test]
        fn test_tls_config_validate() {
            let config = TlsConfig::new();
            assert!(config.validate().is_ok());
        }

        #[test]
        fn test_tls_config_validate_missing_key() {
            let config = TlsConfig::new().with_client_cert(
                std::path::PathBuf::from("/path/to/cert"),
                std::path::PathBuf::from("/path/to/key"),
            );
            assert!(config.validate().is_ok());
        }

        #[test]
        fn test_proxy_config_new() {
            let config = ProxyConfig::new();
            assert!(config.proxy.is_none());
        }

        #[test]
        fn test_proxy_config_http() {
            let config = ProxyConfig::http("http://proxy.example.com:8080");
            assert!(matches!(config.proxy, Some(ProxyType::Http { .. })));
        }

        #[test]
        fn test_proxy_config_https() {
            let config = ProxyConfig::https("https://proxy.example.com:8080");
            assert!(matches!(config.proxy, Some(ProxyType::Https { .. })));
        }

        #[test]
        fn test_proxy_config_socks5() {
            let config = ProxyConfig::socks5("socks5://proxy.example.com:1080");
            assert!(matches!(config.proxy, Some(ProxyType::Socks5 { .. })));
        }

        #[test]
        fn test_proxy_config_with_auth() {
            let config = ProxyConfig::http("http://proxy.com:8080").with_auth("user", "pass");
            assert!(config.auth.is_some());
        }

        #[test]
        fn test_proxy_config_should_bypass() {
            let config = ProxyConfig::http("http://proxy.com:8080")
                .with_no_proxy(vec!["localhost".to_string(), ".local".to_string()]);
            assert!(config.should_bypass("localhost"));
            assert!(config.should_bypass("example.local"));
        }

        #[test]
        fn test_proxy_config_should_not_bypass() {
            let config = ProxyConfig::http("http://proxy.com:8080")
                .with_no_proxy(vec!["localhost".to_string()]);
            assert!(!config.should_bypass("example.com"));
        }

        #[test]
        fn test_proxy_config_validate() {
            let config = ProxyConfig::http("http://proxy.com:8080");
            assert!(config.validate().is_ok());
        }

        #[test]
        fn test_secure_credential_store() {
            let mut store = SecureCredentialStore::new();
            store.store("key1", "value1");
            assert_eq!(store.retrieve("key1"), Some("value1".to_string()));
        }

        #[test]
        fn test_secure_credential_store_remove() {
            let mut store = SecureCredentialStore::new();
            store.store("key1", "value1");
            assert_eq!(store.remove("key1"), Some("value1".to_string()));
            assert_eq!(store.retrieve("key1"), None);
        }

        #[test]
        fn test_secure_credential_store_clear() {
            let mut store = SecureCredentialStore::new();
            store.store("key1", "value1");
            store.store("key2", "value2");
            store.clear();
            assert!(store.is_empty());
        }
    }

    mod cookie_tests {
        use super::*;

        #[test]
        fn test_cookie_creation() {
            let cookie = Cookie::new("name", "value");
            assert_eq!(cookie.name, "name");
            assert_eq!(cookie.value, "value");
        }

        #[test]
        fn test_cookie_with_domain() {
            let cookie = Cookie::new("name", "value").with_domain("example.com");
            assert_eq!(cookie.domain, Some("example.com".to_string()));
        }

        #[test]
        fn test_cookie_with_path() {
            let cookie = Cookie::new("name", "value").with_path("/api");
            assert_eq!(cookie.path, Some("/api".to_string()));
        }

        #[test]
        fn test_cookie_secure() {
            let cookie = Cookie::new("name", "value").secure();
            assert!(cookie.secure);
        }

        #[test]
        fn test_cookie_http_only() {
            let cookie = Cookie::new("name", "value").http_only();
            assert!(cookie.http_only);
        }

        #[test]
        fn test_cookie_same_site() {
            let cookie = Cookie::new("name", "value").with_same_site(SameSite::Strict);
            assert_eq!(cookie.same_site, SameSite::Strict);
        }

        #[test]
        fn test_cookie_matches_domain() {
            let cookie = Cookie::new("name", "value").with_domain("example.com");
            assert!(cookie.matches_domain("example.com"));
            assert!(!cookie.matches_domain("other.com"));
        }

        #[test]
        fn test_cookie_matches_domain_wildcard() {
            let cookie = Cookie::new("name", "value").with_domain(".example.com");
            assert!(cookie.matches_domain("sub.example.com"));
            assert!(cookie.matches_domain("example.com"));
        }

        #[test]
        fn test_cookie_matches_path() {
            let cookie = Cookie::new("name", "value").with_path("/api");
            assert!(cookie.matches_path("/api"));
            assert!(cookie.matches_path("/api/v1"));
            assert!(!cookie.matches_path("/other"));
        }

        #[test]
        fn test_cookie_to_header_value() {
            let cookie = Cookie::new("session", "abc123");
            assert_eq!(cookie.to_header_value(), "session=abc123");
        }

        #[test]
        fn test_cookie_jar_add() {
            let mut jar = CookieJar::new();
            let cookie = Cookie::new("name", "value");
            jar.add(cookie.clone());
            assert_eq!(jar.get("name").unwrap().value, "value");
        }

        #[test]
        fn test_cookie_jar_remove() {
            let mut jar = CookieJar::new();
            jar.add(Cookie::new("name", "value"));
            jar.remove("name");
            assert!(jar.get("name").is_none());
        }

        #[test]
        fn test_cookie_jar_clear() {
            let mut jar = CookieJar::new();
            jar.add(Cookie::new("name1", "value1"));
            jar.add(Cookie::new("name2", "value2"));
            jar.clear();
            assert!(jar.is_empty());
        }

        #[test]
        fn test_cookie_jar_get_for_url() {
            let mut jar = CookieJar::new();
            let cookie = Cookie::new("session", "abc")
                .with_domain("example.com")
                .with_path("/api");
            jar.add(cookie);
            let cookies = jar.get_for_url("example.com", "/api");
            assert_eq!(cookies.len(), 1);
        }

        #[test]
        fn test_cookie_jar_get_cookie_header() {
            let mut jar = CookieJar::new();
            jar.add(
                Cookie::new("session", "abc")
                    .with_domain("example.com")
                    .with_path("/"),
            );
            let header = jar.get_cookie_header("example.com", "/api");
            assert!(header.is_some());
            assert!(header.unwrap().contains("session=abc"));
        }

        #[test]
        fn test_cookie_jar_len() {
            let mut jar = CookieJar::new();
            jar.add(Cookie::new("name1", "value1"));
            jar.add(Cookie::new("name2", "value2"));
            assert_eq!(jar.len(), 2);
        }
    }

    mod compression_tests {
        use super::*;

        #[test]
        fn test_compression_codec_gzip() {
            assert_eq!(CompressionCodec::Gzip.as_header_value(), "gzip");
        }

        #[test]
        fn test_compression_codec_deflate() {
            assert_eq!(CompressionCodec::Deflate.as_header_value(), "deflate");
        }

        #[test]
        fn test_compression_codec_brotli() {
            assert_eq!(CompressionCodec::Brotli.as_header_value(), "br");
        }

        #[test]
        fn test_compression_codec_from_header_gzip() {
            assert_eq!(
                CompressionCodec::from_header_value("gzip"),
                Some(CompressionCodec::Gzip)
            );
        }

        #[test]
        fn test_compression_codec_from_header_deflate() {
            assert_eq!(
                CompressionCodec::from_header_value("deflate"),
                Some(CompressionCodec::Deflate)
            );
        }

        #[test]
        fn test_compression_codec_from_header_br() {
            assert_eq!(
                CompressionCodec::from_header_value("br"),
                Some(CompressionCodec::Brotli)
            );
        }

        #[test]
        fn test_compression_codec_case_insensitive() {
            assert_eq!(
                CompressionCodec::from_header_value("GZIP"),
                Some(CompressionCodec::Gzip)
            );
        }

        #[test]
        fn test_compression_codec_unknown() {
            assert_eq!(CompressionCodec::from_header_value("unknown"), None);
        }

        #[test]
        fn test_compression_config_new() {
            let config = CompressionConfig::new();
            assert!(config.enabled);
            assert_eq!(config.codecs.len(), 1);
            assert_eq!(config.min_size_bytes, 1024);
        }

        #[test]
        fn test_compression_config_disabled() {
            let config = CompressionConfig::disabled();
            assert!(!config.enabled);
        }

        #[test]
        fn test_compression_config_with_codecs() {
            let config = CompressionConfig::new()
                .with_codecs(vec![CompressionCodec::Gzip, CompressionCodec::Brotli]);
            assert_eq!(config.codecs.len(), 2);
        }

        #[test]
        fn test_compression_config_with_min_size() {
            let config = CompressionConfig::new().with_min_size(2048);
            assert_eq!(config.min_size_bytes, 2048);
        }

        #[test]
        fn test_accept_encoding_header() {
            let config = CompressionConfig::new()
                .with_codecs(vec![CompressionCodec::Gzip, CompressionCodec::Brotli]);
            let header = config.accept_encoding_header();
            assert!(header.contains("gzip"));
            assert!(header.contains("br"));
        }
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_tls_with_cert_pinning() {
            let config =
                TlsConfig::new().with_cert_pinning(vec!["pin1".to_string(), "pin2".to_string()]);
            assert!(config.validate().is_ok());
            assert_eq!(config.cert_pinning.unwrap().len(), 2);
        }

        #[test]
        fn test_proxy_with_bypass_list() {
            let config = ProxyConfig::http("http://proxy.com").with_no_proxy(vec![
                "localhost".to_string(),
                ".local".to_string(),
                "127.0.0.1".to_string(),
            ]);
            assert!(config.should_bypass("localhost"));
            assert!(config.should_bypass("example.local"));
            assert!(!config.should_bypass("example.com"));
        }

        #[test]
        fn test_multiple_auth_types() {
            let basic = Auth::basic("user", "pass");
            let bearer = Auth::bearer("token");
            let digest = Auth::digest("user", "pass");
            let oauth2 = Auth::oauth2("token");
            let kerberos = Auth::kerberos("user@REALM");

            assert!(basic.validate().is_ok());
            assert!(bearer.validate().is_ok());
            assert!(digest.validate().is_ok());
            assert!(oauth2.validate().is_ok());
            assert!(kerberos.validate().is_ok());
        }

        #[test]
        fn test_cookie_persistence_workflow() {
            let mut jar = CookieJar::new();
            let cookie1 = Cookie::new("session", "abc123")
                .with_domain("example.com")
                .with_path("/")
                .secure()
                .http_only();
            let cookie2 = Cookie::new("tracking", "xyz789")
                .with_domain(".example.com")
                .with_path("/api");

            jar.add(cookie1);
            jar.add(cookie2);

            assert_eq!(jar.len(), 2);
            let cookies = jar.get_for_url("example.com", "/api");
            assert_eq!(cookies.len(), 2);
        }

        #[test]
        fn test_compression_config_multiple_codecs() {
            let config = CompressionConfig::new().with_codecs(vec![
                CompressionCodec::Gzip,
                CompressionCodec::Brotli,
                CompressionCodec::Deflate,
            ]);
            let header = config.accept_encoding_header();
            assert!(header.contains("gzip"));
            assert!(header.contains("br"));
            assert!(header.contains("deflate"));
        }

        #[test]
        fn test_oauth2_refresh_workflow() {
            let mut auth = Auth::oauth2_with_refresh("old_token", "refresh_token", 3600);
            match auth {
                Auth::OAuth2 {
                    ref mut expires_in, ..
                } => {
                    *expires_in = Some(0);
                }
                _ => panic!("Expected OAuth2"),
            }
            assert!(auth.is_expired());
        }

        #[test]
        fn test_tls_version_ordering() {
            let tls12 = TlsConfig::new().with_min_tls_version(TlsVersion::TLS1_2);
            let tls13 = TlsConfig::new().with_min_tls_version(TlsVersion::TLS1_3);
            assert_eq!(tls12.min_tls_version, TlsVersion::TLS1_2);
            assert_eq!(tls13.min_tls_version, TlsVersion::TLS1_3);
        }
    }

    mod error_scenario_tests {
        use super::*;

        #[test]
        fn test_empty_basic_auth_credentials() {
            let auth = Auth::Basic {
                username: "".to_string(),
                password: "".to_string(),
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_empty_digest_credentials() {
            let auth = Auth::Digest {
                username: "".to_string(),
                password: "".to_string(),
                realm: None,
                nonce: None,
                uri: None,
                qop: None,
                nc: 0,
                cnonce: None,
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_empty_bearer_token() {
            let auth = Auth::Bearer {
                token: "".to_string(),
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_empty_oauth2_token() {
            let auth = Auth::OAuth2 {
                token: "".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: None,
                refresh_token: None,
            };
            assert!(auth.validate().is_err());
        }

        #[test]
        fn test_empty_proxy_url() {
            let config = ProxyConfig {
                proxy: Some(ProxyType::Http {
                    url: "".to_string(),
                }),
                no_proxy: vec![],
                auth: None,
            };
            assert!(config.validate().is_err());
        }

        #[test]
        fn test_expired_oauth2() {
            let auth = Auth::OAuth2 {
                token: "token".to_string(),
                token_type: "Bearer".to_string(),
                expires_in: Some(0),
                refresh_token: None,
            };
            assert!(auth.is_expired());
        }

        #[test]
        fn test_cookie_domain_mismatch() {
            let cookie = Cookie::new("name", "value").with_domain("example.com");
            assert!(!cookie.matches_domain("other.com"));
        }

        #[test]
        fn test_cookie_path_mismatch() {
            let cookie = Cookie::new("name", "value").with_path("/api");
            assert!(!cookie.matches_path("/other"));
        }

        #[test]
        fn test_proxy_bypass_domain_mismatch() {
            let config = ProxyConfig::http("http://proxy.com")
                .with_no_proxy(vec!["example.com".to_string()]);
            assert!(!config.should_bypass("other.com"));
        }
    }
}
