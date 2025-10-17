use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub verbose: bool,
    pub quiet: bool,
    pub config_path: Option<PathBuf>,
}

impl Config {
    pub fn new(verbose: bool, quiet: bool, config_path: Option<PathBuf>) -> Self {
        Config {
            verbose,
            quiet,
            config_path,
        }
    }

    pub fn should_output(&self) -> bool {
        !self.quiet
    }
}
