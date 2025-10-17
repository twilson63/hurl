use std::collections::HashMap;
use std::time::Instant;

use crate::http::request::RequestBuilder;
use crate::http::response::HttpResponse;
use crate::Result;

#[derive(Debug, Clone)]
pub struct BatchRequest {
    pub id: String,
    pub request: RequestBuilder,
    pub metadata: HashMap<String, String>,
}

impl BatchRequest {
    pub fn new(id: &str, request: RequestBuilder) -> Self {
        BatchRequest {
            id: id.to_string(),
            request,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Debug, Clone)]
pub struct BatchResponse {
    pub id: String,
    pub response: HttpResponse,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct BatchExecutor {
    requests: Vec<BatchRequest>,
    max_concurrent: usize,
    stop_on_error: bool,
}

impl BatchExecutor {
    pub fn new(max_concurrent: usize) -> Self {
        BatchExecutor {
            requests: Vec::new(),
            max_concurrent,
            stop_on_error: false,
        }
    }

    pub fn add_request(mut self, request: BatchRequest) -> Self {
        self.requests.push(request);
        self
    }

    pub fn add_requests(mut self, requests: Vec<BatchRequest>) -> Self {
        self.requests.extend(requests);
        self
    }

    pub fn with_stop_on_error(mut self, stop: bool) -> Self {
        self.stop_on_error = stop;
        self
    }

    pub fn execute<F>(&self, client_fn: F) -> Result<BatchResult>
    where
        F: Fn(RequestBuilder) -> Result<HttpResponse>,
    {
        let start = Instant::now();
        let mut responses = Vec::new();
        let mut stats = BatchStats::default();

        for batch_req in &self.requests {
            match client_fn(batch_req.request.clone()) {
                Ok(response) => {
                    let success = response.is_success();
                    responses.push(BatchResponse {
                        id: batch_req.id.clone(),
                        response: response.clone(),
                        success,
                        error: None,
                    });
                    stats.successful += 1;
                    if success {
                        stats.succeeded += 1;
                    }
                }
                Err(e) => {
                    responses.push(BatchResponse {
                        id: batch_req.id.clone(),
                        response: HttpResponse::new(0, HashMap::new(), "".to_string()),
                        success: false,
                        error: Some(e.to_string()),
                    });
                    stats.failed += 1;

                    if self.stop_on_error {
                        break;
                    }
                }
            }
        }

        stats.duration = start.elapsed();
        stats.total = responses.len();

        Ok(BatchResult { responses, stats })
    }

    pub fn request_count(&self) -> usize {
        self.requests.len()
    }
}

impl Default for BatchExecutor {
    fn default() -> Self {
        Self::new(10)
    }
}

#[derive(Debug, Clone)]
pub struct BatchStats {
    pub total: usize,
    pub succeeded: usize,
    pub successful: usize,
    pub failed: usize,
    pub duration: std::time::Duration,
}

impl Default for BatchStats {
    fn default() -> Self {
        BatchStats {
            total: 0,
            succeeded: 0,
            successful: 0,
            failed: 0,
            duration: std::time::Duration::from_secs(0),
        }
    }
}

impl BatchStats {
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.succeeded as f64 / self.total as f64) * 100.0
        }
    }

    pub fn error_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.failed as f64 / self.total as f64) * 100.0
        }
    }

    pub fn avg_time_per_request(&self) -> std::time::Duration {
        if self.total == 0 {
            std::time::Duration::from_secs(0)
        } else {
            self.duration / self.total as u32
        }
    }
}

#[derive(Debug)]
pub struct BatchResult {
    pub responses: Vec<BatchResponse>,
    pub stats: BatchStats,
}

impl BatchResult {
    pub fn get_response(&self, id: &str) -> Option<&BatchResponse> {
        self.responses.iter().find(|r| r.id == id)
    }

    pub fn get_all_successful(&self) -> Vec<&BatchResponse> {
        self.responses.iter().filter(|r| r.success).collect()
    }

    pub fn get_all_failed(&self) -> Vec<&BatchResponse> {
        self.responses.iter().filter(|r| !r.success).collect()
    }
}
