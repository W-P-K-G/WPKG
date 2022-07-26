use reqwest::{Method, Response};

pub async fn send_api_request(method: Method, url: &str) -> anyhow::Result<Response> {
    // create http client
    let client = reqwest::Client::new();

    // create request builder
    let req = client.request(method, url);

    // build request
    let req = req.build()?;

    // send request
    Ok(client.execute(req).await?)
}
