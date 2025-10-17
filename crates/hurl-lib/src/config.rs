use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub timeout: u64,
    pub follow_redirects: bool,
    pub verify_ssl: bool,
    pub proxy: Option<String>,
    pub user_agent: Option<String>,
    pub headers: std::collections::HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: 30,
            follow_redirects: true,
            verify_ssl: true,
            proxy: None,
            user_agent: None,
            headers: std::collections::HashMap::new(),
        }
    }
}

impl Config {
    pub fn from_file(path: PathBuf) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn to_file(&self, path: PathBuf) -> crate::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
