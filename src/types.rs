use anyhow::anyhow;
use reqwest::Method;
use serde::{Serialize, Deserialize};

use crate::{globals::JSON_WALLETS_URL, send_api_request};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub coin: String,
    pub id: String,
    pub referral: String,
}

pub type Wallets = Vec<Wallet>;

impl Wallet {
    pub fn parse(data: &str) -> serde_json::Result<Wallets> {
        serde_json::from_str(data)
    }

    pub async fn get() -> anyhow::Result<Wallets> {
        let res = send_api_request(Method::GET, JSON_WALLETS_URL).await?;

        // get response http code
        let status = res.status();

        // check if an error has occurred
        if status.is_client_error() || status.is_server_error() {
            let body = res.text().await?;

            return Err(anyhow!("Server returned non-successful response: {} (http code: {})", body, status));
        }

        Ok(res.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = r#"
        [
            {
                "coin": "ETH",
                "id": "0x180075dBfBe69E91508174daC16cbD022ADfE52B",
                "referral": ""
            },
            {
                "coin": "ETC",
                "id": "0xbCb8b2cB79b0AF1AbCA289D5E837328866408b08",
                "referral": ""
            },
            {
                "coin": "TRX",
                "id": "TY9r7wFuWhvvmwvGD88pBdfxuZ2hRmy5Ko",
                "referral": "6mww-jqa4"
            }
        ]"#;

        Wallet::parse(data).unwrap();
    }

    #[tokio::test]
    async fn test_get_from_api() {
        Wallet::get().await.unwrap();
    }
}
