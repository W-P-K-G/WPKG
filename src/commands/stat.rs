use super::prelude::*;
use crate::utils;

pub struct Stat;

#[async_trait]
impl Command for Stat {
    fn name(&self) -> String {
        crypto!("stat")
    }

    fn help(&self) -> String {
        crypto!("sending pc stats (CPU, Memory and Swap)")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(utils::stat())
    }
}
