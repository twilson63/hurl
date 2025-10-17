use crate::http::response::HttpResponse;
use crate::{Error, Result};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Variables {
    vars: HashMap<String, Value>,
}

impl Variables {
    pub fn new() -> Self {
        Variables {
            vars: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.vars.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.vars.get(key)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.vars.get(key).and_then(|v| {
            if let Value::String(s) = v {
                Some(s.clone())
            } else {
                v.as_str().map(|s| s.to_string())
            }
        })
    }

    pub fn get_number(&self, key: &str) -> Option<i64> {
        self.vars.get(key).and_then(|v| v.as_i64())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.vars.get(key).and_then(|v| v.as_bool())
    }

    pub fn set_from_env(&mut self, key: &str) -> Result<()> {
        let value =
            std::env::var(key).map_err(|_| Error::Config(format!("env var not found: {}", key)))?;
        self.set(key, Value::String(value));
        Ok(())
    }

    pub fn all(&self) -> HashMap<String, Value> {
        self.vars.clone()
    }

    pub fn clear(&mut self) {
        self.vars.clear();
    }
}

impl Default for Variables {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ExtractionRule {
    pub name: String,
    pub rule_type: ExtractionType,
}

#[derive(Debug, Clone)]
pub enum ExtractionType {
    JsonPath(String),
    Header(String),
    Status,
    Duration,
    Size,
}

impl ExtractionRule {
    pub fn json_path(name: &str, path: &str) -> Self {
        ExtractionRule {
            name: name.to_string(),
            rule_type: ExtractionType::JsonPath(path.to_string()),
        }
    }

    pub fn header(name: &str, header_name: &str) -> Self {
        ExtractionRule {
            name: name.to_string(),
            rule_type: ExtractionType::Header(header_name.to_string()),
        }
    }

    pub fn status(name: &str) -> Self {
        ExtractionRule {
            name: name.to_string(),
            rule_type: ExtractionType::Status,
        }
    }

    pub fn duration(name: &str) -> Self {
        ExtractionRule {
            name: name.to_string(),
            rule_type: ExtractionType::Duration,
        }
    }

    pub fn size(name: &str) -> Self {
        ExtractionRule {
            name: name.to_string(),
            rule_type: ExtractionType::Size,
        }
    }
}

pub struct Extractor;

impl Extractor {
    pub fn extract_json_path(json: &str, path: &str) -> Result<Value> {
        let value: Value = serde_json::from_str(json)
            .map_err(|e| Error::Parse(format!("failed to parse json: {}", e)))?;

        Self::navigate_path(&value, path)
    }

    fn navigate_path(value: &Value, path: &str) -> Result<Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;

        for part in parts {
            if part.is_empty() {
                continue;
            }

            if let Some(idx_start) = part.find('[') {
                let key = &part[..idx_start];
                if !key.is_empty() {
                    current = &current[key];
                }

                let idx_end = part
                    .find(']')
                    .ok_or_else(|| Error::Parse("malformed array index".to_string()))?;
                let idx_str = &part[idx_start + 1..idx_end];
                let idx: usize = idx_str
                    .parse()
                    .map_err(|_| Error::Parse("invalid array index".to_string()))?;
                current = &current[idx];
            } else {
                current = &current[part];
            }

            if current.is_null() {
                return Err(Error::Parse(format!("path not found: {}", path)));
            }
        }

        Ok(current.clone())
    }

    pub fn extract_header(response: &HttpResponse, name: &str) -> Result<String> {
        response
            .header(name)
            .map(|s| s.to_string())
            .ok_or_else(|| Error::Http(format!("header not found: {}", name)))
    }

    pub fn apply_extractions(
        response: &HttpResponse,
        rules: &[ExtractionRule],
    ) -> Result<Variables> {
        let mut vars = Variables::new();

        for rule in rules {
            let value = match &rule.rule_type {
                ExtractionType::JsonPath(path) => Self::extract_json_path(&response.body, path)?,
                ExtractionType::Header(name) => {
                    Value::String(Self::extract_header(response, name)?)
                }
                ExtractionType::Status => serde_json::Number::from(response.status as i64).into(),
                ExtractionType::Duration => {
                    let millis = response.duration.as_millis() as i64;
                    serde_json::Number::from(millis).into()
                }
                ExtractionType::Size => {
                    let size = response.body.len() as i64;
                    serde_json::Number::from(size).into()
                }
            };

            vars.set(&rule.name, value);
        }

        Ok(vars)
    }
}

#[derive(Debug)]
pub struct RequestChain {
    requests: Vec<ChainRequest>,
}

#[derive(Debug, Clone)]
pub struct ChainRequest {
    pub method: String,
    pub url_template: String,
    pub headers: HashMap<String, String>,
    pub body_template: Option<String>,
    pub extractions: Vec<ExtractionRule>,
    pub name: Option<String>,
}

impl ChainRequest {
    pub fn new(method: &str, url: &str) -> Self {
        ChainRequest {
            method: method.to_string(),
            url_template: url.to_string(),
            headers: HashMap::new(),
            body_template: None,
            extractions: Vec::new(),
            name: None,
        }
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body_template = Some(body.to_string());
        self
    }

    pub fn extract(mut self, rule: ExtractionRule) -> Self {
        self.extractions.push(rule);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    fn substitute_variables(&self, vars: &Variables) -> Self {
        let url = Self::interpolate(&self.url_template, vars);
        let body = self
            .body_template
            .as_ref()
            .map(|b| Self::interpolate(b, vars));

        let headers = self
            .headers
            .iter()
            .map(|(k, v)| (k.clone(), Self::interpolate(v, vars)))
            .collect();

        ChainRequest {
            method: self.method.clone(),
            url_template: url,
            headers,
            body_template: body,
            extractions: self.extractions.clone(),
            name: self.name.clone(),
        }
    }

    fn interpolate(template: &str, vars: &Variables) -> String {
        let mut result = template.to_string();

        for (key, value) in vars.all().iter() {
            let placeholder = format!("${{{}}}", key);
            let value_str = match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &value_str);
        }

        result
    }
}

impl RequestChain {
    pub fn new() -> Self {
        RequestChain {
            requests: Vec::new(),
        }
    }

    pub fn add_request(mut self, request: ChainRequest) -> Self {
        self.requests.push(request);
        self
    }

    pub fn execute_sync<F>(&self, client_fn: F) -> Result<ChainResult>
    where
        F: Fn(&str, &str, HashMap<String, String>, Option<String>) -> Result<HttpResponse>,
    {
        let mut vars = Variables::new();
        let mut context = ChainContext::new();

        for req in &self.requests {
            let substituted = req.substitute_variables(&vars);
            let name = substituted
                .name
                .clone()
                .unwrap_or_else(|| format!("req_{}", context.steps.len()));
            let extractions = substituted.extractions.clone();

            let response = client_fn(
                &substituted.method,
                &substituted.url_template,
                substituted.headers.clone(),
                substituted.body_template.clone(),
            )?;

            let extracted = Extractor::apply_extractions(&response, &extractions)?;
            let extracted_vars = extracted.vars.clone();
            vars.vars.extend(extracted.vars);

            let step = ChainStep {
                name,
                request: substituted,
                response: response.clone(),
                extracted_vars,
            };

            context.steps.push(step);
        }

        Ok(ChainResult {
            context,
            variables: vars,
            success: true,
        })
    }
}

impl Default for RequestChain {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ChainStep {
    pub name: String,
    pub request: ChainRequest,
    pub response: HttpResponse,
    pub extracted_vars: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct ChainContext {
    pub steps: Vec<ChainStep>,
}

impl ChainContext {
    pub fn new() -> Self {
        ChainContext { steps: Vec::new() }
    }

    pub fn get_step(&self, index: usize) -> Option<&ChainStep> {
        self.steps.get(index)
    }

    pub fn get_step_by_name(&self, name: &str) -> Option<&ChainStep> {
        self.steps.iter().find(|s| s.name == name)
    }
}

impl Default for ChainContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct ChainResult {
    pub context: ChainContext,
    pub variables: Variables,
    pub success: bool,
}

impl ChainResult {
    pub fn get_final_response(&self) -> Option<&HttpResponse> {
        self.context.steps.last().map(|s| &s.response)
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}
