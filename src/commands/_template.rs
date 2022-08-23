use super::prelude::*;

pub struct Stat;

#[async_trait]
impl Command for Stat {
    fn name(&self) -> &'static str {
        crypto!("cmd")
    }

    fn help(&self) -> &'static str {
        crypto!("Help")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        Ok(client.send("send")?)
    }
}
