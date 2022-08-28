use super::prelude::*;

pub struct Test;

#[async_trait]
impl Command for Test {
    fn name(&self) -> &'static str {
        encode!("cmd")
    }

    fn help(&self) -> &'static str {
        encode!("Help")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        Ok(client.send(crypto!("message"))?)
    }
}
