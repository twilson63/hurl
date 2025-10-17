use crate::http::response::HttpResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub response: HttpResponse,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

impl CachedResponse {
    pub fn is_expired(&self) -> bool {
        let elapsed = Instant::now()
            .checked_duration_since(Instant::now() - Duration::from_secs(self.ttl_seconds));

        elapsed.is_none_or(|_| false)
    }
}

#[derive(Debug)]
pub struct ResponseCache {
    cache: HashMap<String, (HttpResponse, Instant, Duration)>,
    hits: usize,
    misses: usize,
    policies: CachePolicies,
}

#[derive(Debug, Clone)]
pub struct CachePolicies {
    pub default_ttl: Duration,
    pub max_entries: usize,
    pub cache_by_method: bool,
    pub cache_successful_only: bool,
}

impl Default for CachePolicies {
    fn default() -> Self {
        CachePolicies {
            default_ttl: Duration::from_secs(300),
            max_entries: 1000,
            cache_by_method: true,
            cache_successful_only: true,
        }
    }
}

impl ResponseCache {
    pub fn new(policies: CachePolicies) -> Self {
        ResponseCache {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
            policies,
        }
    }

    pub fn with_default_policies() -> Self {
        Self::new(CachePolicies::default())
    }

    fn generate_key(&self, url: &str, method: &str) -> String {
        if self.policies.cache_by_method {
            format!("{}::{}", method, url)
        } else {
            url.to_string()
        }
    }

    pub fn get(&mut self, url: &str, method: &str) -> Option<HttpResponse> {
        let key = self.generate_key(url, method);

        if let Some((response, created_at, ttl)) = self.cache.get(&key) {
            if created_at.elapsed() < *ttl {
                self.hits += 1;
                return Some(response.clone());
            } else {
                self.cache.remove(&key);
            }
        }

        self.misses += 1;
        None
    }

    pub fn put(&mut self, url: &str, method: &str, response: HttpResponse) {
        if self.policies.cache_successful_only && !response.is_success() {
            return;
        }

        if self.cache.len() >= self.policies.max_entries {
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }

        let key = self.generate_key(url, method);
        self.cache
            .insert(key, (response, Instant::now(), self.policies.default_ttl));
    }

    pub fn put_with_ttl(&mut self, url: &str, method: &str, response: HttpResponse, ttl: Duration) {
        if self.policies.cache_successful_only && !response.is_success() {
            return;
        }

        if self.cache.len() >= self.policies.max_entries {
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }

        let key = self.generate_key(url, method);
        self.cache.insert(key, (response, Instant::now(), ttl));
    }

    pub fn invalidate(&mut self, url: &str, method: Option<&str>) {
        if let Some(m) = method {
            let key = self.generate_key(url, m);
            self.cache.remove(&key);
        } else {
            self.cache
                .retain(|k, _| !k.ends_with(&format!("::{}", url)));
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    pub fn stats(&self) -> CacheStats {
        let total = self.hits + self.misses;
        let hit_rate = if total > 0 {
            (self.hits as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        CacheStats {
            hits: self.hits,
            misses: self.misses,
            total_requests: total,
            hit_rate,
            size: self.cache.len(),
        }
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }

    pub fn set_policies(&mut self, policies: CachePolicies) {
        self.policies = policies;
    }
}

impl Clone for ResponseCache {
    fn clone(&self) -> Self {
        ResponseCache {
            cache: self.cache.clone(),
            hits: self.hits,
            misses: self.misses,
            policies: self.policies.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub total_requests: usize,
    pub hit_rate: f64,
    pub size: usize,
}
