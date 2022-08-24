use super::prelude::*;

pub struct Ping;

#[async_trait]
impl Command for Ping {
    fn name(&self) -> &'static str {
        encode!("ping")
    }

    fn help(&self) -> &'static str {
        encode!("command for pinger")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        Ok(client.send("ping-received")?)
    }
}
