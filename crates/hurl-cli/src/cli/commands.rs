use crate::cli::config::Config;
use crate::cli::parser::RequestConfig;
use anyhow::Result;
use reqwest::Client;
use std::path::PathBuf;
use std::time::Duration;

pub async fn handle_get(
    url: &str,
    headers: Vec<String>,
    auth: Option<String>,
    timeout: Option<u64>,
    output: Option<PathBuf>,
    config: &Config,
) -> Result<()> {
    let request_config = RequestConfig::new(url, headers, auth, timeout, None)?;

    if config.verbose {
        eprintln!("[VERBOSE] GET {}", request_config.url);
    }

    let client = build_client(request_config.timeout)?;
    let mut request = client.get(&request_config.url);

    request = apply_headers(request, request_config.headers);
    request = apply_auth(request, request_config.auth);

    let response = request.send().await?;

    if config.should_output() {
        print_response(response, output, config).await?;
    }

    Ok(())
}

pub async fn handle_post(
    url: &str,
    headers: Vec<String>,
    data: Option<String>,
    auth: Option<String>,
    timeout: Option<u64>,
    output: Option<PathBuf>,
    config: &Config,
) -> Result<()> {
    let request_config = RequestConfig::new(url, headers, auth, timeout, data)?;

    if config.verbose {
        eprintln!("[VERBOSE] POST {}", request_config.url);
        if let Some(body) = &request_config.body {
            eprintln!("[VERBOSE] Body: {}", body);
        }
    }

    let client = build_client(request_config.timeout)?;
    let mut request = client.post(&request_config.url);

    request = apply_headers(request, request_config.headers);
    request = apply_auth(request, request_config.auth);

    if let Some(body) = request_config.body {
        request = request.body(body);
    }

    let response = request.send().await?;

    if config.should_output() {
        print_response(response, output, config).await?;
    }

    Ok(())
}

pub async fn handle_put(
    url: &str,
    headers: Vec<String>,
    data: Option<String>,
    auth: Option<String>,
    timeout: Option<u64>,
    output: Option<PathBuf>,
    config: &Config,
) -> Result<()> {
    let request_config = RequestConfig::new(url, headers, auth, timeout, data)?;

    if config.verbose {
        eprintln!("[VERBOSE] PUT {}", request_config.url);
        if let Some(body) = &request_config.body {
            eprintln!("[VERBOSE] Body: {}", body);
        }
    }

    let client = build_client(request_config.timeout)?;
    let mut request = client.put(&request_config.url);

    request = apply_headers(request, request_config.headers);
    request = apply_auth(request, request_config.auth);

    if let Some(body) = request_config.body {
        request = request.body(body);
    }

    let response = request.send().await?;

    if config.should_output() {
        print_response(response, output, config).await?;
    }

    Ok(())
}

pub async fn handle_delete(
    url: &str,
    headers: Vec<String>,
    auth: Option<String>,
    timeout: Option<u64>,
    output: Option<PathBuf>,
    config: &Config,
) -> Result<()> {
    let request_config = RequestConfig::new(url, headers, auth, timeout, None)?;

    if config.verbose {
        eprintln!("[VERBOSE] DELETE {}", request_config.url);
    }

    let client = build_client(request_config.timeout)?;
    let mut request = client.delete(&request_config.url);

    request = apply_headers(request, request_config.headers);
    request = apply_auth(request, request_config.auth);

    let response = request.send().await?;

    if config.should_output() {
        print_response(response, output, config).await?;
    }

    Ok(())
}

pub async fn handle_patch(
    url: &str,
    headers: Vec<String>,
    data: Option<String>,
    auth: Option<String>,
    timeout: Option<u64>,
    output: Option<PathBuf>,
    config: &Config,
) -> Result<()> {
    let request_config = RequestConfig::new(url, headers, auth, timeout, data)?;

    if config.verbose {
        eprintln!("[VERBOSE] PATCH {}", request_config.url);
        if let Some(body) = &request_config.body {
            eprintln!("[VERBOSE] Body: {}", body);
        }
    }

    let client = build_client(request_config.timeout)?;
    let mut request = client.patch(&request_config.url);

    request = apply_headers(request, request_config.headers);
    request = apply_auth(request, request_config.auth);

    if let Some(body) = request_config.body {
        request = request.body(body);
    }

    let response = request.send().await?;

    if config.should_output() {
        print_response(response, output, config).await?;
    }

    Ok(())
}

pub async fn handle_head(
    url: &str,
    headers: Vec<String>,
    auth: Option<String>,
    timeout: Option<u64>,
    config: &Config,
) -> Result<()> {
    let request_config = RequestConfig::new(url, headers, auth, timeout, None)?;

    if config.verbose {
        eprintln!("[VERBOSE] HEAD {}", request_config.url);
    }

    let client = build_client(request_config.timeout)?;
    let mut request = client.head(&request_config.url);

    request = apply_headers(request, request_config.headers);
    request = apply_auth(request, request_config.auth);

    let response = request.send().await?;

    if config.should_output() {
        println_response_headers(&response, config);
    }

    Ok(())
}

pub async fn handle_options(
    url: &str,
    headers: Vec<String>,
    auth: Option<String>,
    timeout: Option<u64>,
    output: Option<PathBuf>,
    config: &Config,
) -> Result<()> {
    let request_config = RequestConfig::new(url, headers, auth, timeout, None)?;

    if config.verbose {
        eprintln!("[VERBOSE] OPTIONS {}", request_config.url);
    }

    let client = build_client(request_config.timeout)?;
    let mut request = client.request(reqwest::Method::OPTIONS, &request_config.url);

    request = apply_headers(request, request_config.headers);
    request = apply_auth(request, request_config.auth);

    let response = request.send().await?;

    if config.should_output() {
        print_response(response, output, config).await?;
    }

    Ok(())
}

fn build_client(timeout: Option<u64>) -> Result<Client> {
    let mut builder = Client::builder();

    if let Some(secs) = timeout {
        builder = builder.timeout(Duration::from_secs(secs));
    }

    builder
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build HTTP client: {}", e))
}

fn apply_headers(
    mut request: reqwest::RequestBuilder,
    headers: std::collections::HashMap<String, String>,
) -> reqwest::RequestBuilder {
    for (key, value) in headers {
        if let Ok(header_name) = key.parse::<reqwest::header::HeaderName>() {
            if let Ok(header_value) = value.parse::<reqwest::header::HeaderValue>() {
                request = request.header(header_name, header_value);
            }
        }
    }
    request
}

fn apply_auth(
    request: reqwest::RequestBuilder,
    auth: Option<(String, String)>,
) -> reqwest::RequestBuilder {
    match auth {
        Some((username, password)) => request.basic_auth(username, Some(password)),
        None => request,
    }
}

async fn print_response(
    response: reqwest::Response,
    output: Option<PathBuf>,
    config: &Config,
) -> Result<()> {
    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;

    if let Some(path) = output {
        std::fs::write(&path, &body)?;
        if config.verbose {
            eprintln!("[VERBOSE] Response written to {:?}", path);
        }
    } else {
        println!("HTTP/1.1 {}", status);
        for (name, value) in headers.iter() {
            if let Ok(value_str) = value.to_str() {
                println!("{}: {}", name, value_str);
            }
        }
        println!();
        println!("{}", body);
    }

    Ok(())
}

fn println_response_headers(response: &reqwest::Response, config: &Config) {
    let status = response.status();
    let headers = response.headers();

    println!("HTTP/1.1 {}", status);
    for (name, value) in headers.iter() {
        if let Ok(value_str) = value.to_str() {
            println!("{}: {}", name, value_str);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_build_client_without_timeout() {
        let client = build_client(None);
        assert!(client.is_ok());
    }

    #[test]
    fn test_build_client_with_timeout() {
        let client = build_client(Some(30));
        assert!(client.is_ok());
    }

    #[test]
    fn test_apply_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());

        let client = reqwest::Client::new();
        let request = client.get("http://example.com");
        let request = apply_headers(request, headers);

        assert!(request.build().is_ok());
    }

    #[test]
    fn test_apply_auth() {
        let client = reqwest::Client::new();
        let request = client.get("http://example.com");
        let request = apply_auth(request, Some(("user".to_string(), "pass".to_string())));

        assert!(request.build().is_ok());
    }

    #[test]
    fn test_apply_auth_none() {
        let client = reqwest::Client::new();
        let request = client.get("http://example.com");
        let request = apply_auth(request, None);

        assert!(request.build().is_ok());
    }
}
