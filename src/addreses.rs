use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{globals::JSON_ADDRESSES_URL, TCP_BACKUP_IP, TCP_BACKUP_PORT};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Addresses {
    #[serde(rename = "tAddresses")]
    pub tcp: Vec<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub ip: String,
    pub port: u32,
}

impl Default for Address {
    fn default() -> Self {
        Self {
            ip: TCP_BACKUP_IP.to_string(),
            port: TCP_BACKUP_PORT,
        }
    }
}

impl Address {
    pub fn format(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

impl Addresses {
    pub fn parse(data: &str) -> serde_json::Result<Self> {
        serde_json::from_str(data)
    }

    pub async fn get() -> anyhow::Result<Self> {
        let res = reqwest::get(String::from_utf8(JSON_ADDRESSES_URL.to_vec())?).await?;

        // get response http code
        let status = res.status();

        // check if an error has occurred
        if status.is_client_error() || status.is_server_error() {
            let body = res.text().await?;

            return Err(anyhow!(
                "Server returned non-successful response: {} (http code: {})",
                body,
                status
            ));
        }

        Ok(res.json().await?)
    }
}
