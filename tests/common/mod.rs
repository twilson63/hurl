pub fn setup_test_client() -> reqwest::Client {
    reqwest::Client::new()
}

pub async fn make_request(
    client: &reqwest::Client,
    method: &str,
    url: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    match method {
        "GET" => client.get(url).send().await,
        "POST" => client.post(url).send().await,
        "PUT" => client.put(url).send().await,
        "DELETE" => client.delete(url).send().await,
        _ => client.get(url).send().await,
    }
}
