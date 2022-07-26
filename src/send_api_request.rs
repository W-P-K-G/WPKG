use reqwest::Response;

pub async fn send_api_request(url: &str) -> anyhow::Result<Response> {
    // send request
    let res = reqwest::get(url).await?;

    Ok(res)
}
