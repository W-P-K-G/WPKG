use super::prelude::*;

pub struct Ping;

#[async_trait]
impl Command for Ping {
    fn name(&self) -> String {
        crypto!("ping")
    }

    fn help(&self) -> String {
        crypto!("command for pinger")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(crypto!("ping-received"))
    }
}
