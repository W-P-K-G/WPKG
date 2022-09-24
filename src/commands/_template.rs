use super::prelude::*;

pub struct Test;

#[async_trait]
impl Command for Test {
    fn name(&self) -> String {
        crypto!("cmd")
    }

    fn help(&self) -> String {
        crypto!("Help")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(crypto!("message"))
    }
}
