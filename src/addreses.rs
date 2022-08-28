use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use wpkg_crypto::decode;

use crate::{crypto, globals::JSON_ADDRESSES_URL, TCP_BACKUP_IP, TCP_BACKUP_PORT};

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
            ip: decode(TCP_BACKUP_IP),
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
    pub async fn get() -> anyhow::Result<Self> {
        let uri = decode(JSON_ADDRESSES_URL);

        let res = reqwest::get(uri).await?;

        // get response http code
        let status = res.status();

        // check if an error has occurred
        if status.is_client_error() || status.is_server_error() {
            let body = res.text().await?;

            return Err(anyhow!(
                "{body_msg}: {body} ({status_msg}: {status})",
                body_msg = crypto!("Server returned non-successful response"),
                status_msg = crypto!("http code")
            ));
        }

        Ok(res.json().await?)
    }
}
