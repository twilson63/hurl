pub mod batch;
pub mod config;
pub mod error;
pub mod http;
pub mod storage;
pub mod test;
pub mod utils;

#[cfg(test)]
mod integration_tests;

pub use error::{Error, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
