use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{globals::JSON_ADRESSES_URL, TCP_BACKUP_IP, TCP_BACKUP_PORT};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Adresses {
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

impl Adresses {
    pub fn parse(data: &str) -> serde_json::Result<Self> {
        serde_json::from_str(data)
    }

    pub async fn get() -> anyhow::Result<Self> {
        let res = reqwest::get(JSON_ADRESSES_URL).await?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let data = r#"
        {
            "tAddresses": [
                {
                "ip": "136.243.156.104",
                "port": 3217
                },
                {
                "ip": "147.185.221.212",
                "port": 16871
                }
            ],
            "uAddresses": [
                {
                "ip": "136.243.156.104",
                "port": 3218
                },
                {
                "ip": "147.185.221.212",
                "port": 16870
                }
            ]
        }"#;

        Adresses::parse(data).unwrap();
    }

    #[tokio::test]
    async fn test_get_from_api() {
        Adresses::get().await.unwrap();
    }
}
