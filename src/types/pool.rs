use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub pool: String,
    pub wallet: String,
}

impl Pool {
    pub fn parse(data: &str) -> serde_json::Result<Self> {
        serde_json::from_str(data)
    }
}
