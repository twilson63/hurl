use crate::http::request::RequestBuilder;
use crate::http::response::HttpResponse;
use crate::Result;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub timestamp: u64,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub status: u16,
    pub response_headers: HashMap<String, String>,
    pub response_body: String,
    pub duration_ms: u128,
    pub tags: Vec<String>,
}

impl HistoryEntry {
    pub fn from_request_response(
        request: &RequestBuilder,
        response: &HttpResponse,
        tags: Vec<String>,
    ) -> Result<Self> {
        request.validate()?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        let body = if let Some(body) = request.body() {
            Some(String::from_utf8_lossy(&body.to_bytes()?).to_string())
        } else {
            None
        };

        Ok(HistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            url: request.url().to_string(),
            method: request.method().to_string(),
            headers: request.headers().clone(),
            body,
            status: response.status,
            response_headers: response.headers.clone(),
            response_body: response.body.clone(),
            duration_ms: response.duration.as_millis(),
            tags,
        })
    }
}

#[derive(Debug)]
pub struct HistoryStore {
    entries: Vec<HistoryEntry>,
}

impl HistoryStore {
    pub fn new() -> Self {
        HistoryStore {
            entries: Vec::new(),
        }
    }

    pub fn save(&mut self, entry: HistoryEntry) -> String {
        let id = entry.id.clone();
        self.entries.push(entry);
        id
    }

    pub fn get(&self, id: &str) -> Option<&HistoryEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    pub fn search(&self, query: &SearchQuery) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| {
                let url_match = query
                    .url
                    .as_ref()
                    .map(|u| e.url.contains(u))
                    .unwrap_or(true);
                let method_match = query
                    .method
                    .as_ref()
                    .map(|m| e.method.eq_ignore_ascii_case(m))
                    .unwrap_or(true);
                let status_match = query.status.map(|s| e.status == s).unwrap_or(true);
                let tag_match = query
                    .tags
                    .as_ref()
                    .map(|tags| tags.iter().any(|t| e.tags.contains(t)))
                    .unwrap_or(true);

                url_match && method_match && status_match && tag_match
            })
            .collect()
    }

    pub fn delete(&mut self, id: &str) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.id == id) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list_all(&self) -> Vec<&HistoryEntry> {
        self.entries.iter().collect()
    }

    pub fn export_to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.entries).map_err(crate::Error::Serialization)
    }

    pub fn import_from_json(&mut self, json: &str) -> Result<()> {
        let entries: Vec<HistoryEntry> =
            serde_json::from_str(json).map_err(crate::Error::Serialization)?;
        self.entries = entries;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

impl Default for HistoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub url: Option<String>,
    pub method: Option<String>,
    pub status: Option<u16>,
    pub tags: Option<Vec<String>>,
}

impl SearchQuery {
    pub fn new() -> Self {
        SearchQuery {
            url: None,
            method: None,
            status: None,
            tags: None,
        }
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = Some(method.to_string());
        self
    }

    pub fn status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self::new()
    }
}
