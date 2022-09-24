use super::prelude::*;
use crate::globals;

pub struct Version;

#[async_trait]
impl Command for Version {
    fn name(&self) -> String {
        crypto!("version")
    }

    fn help(&self) -> String {
        crypto!("get version of the client")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send(globals::CURRENT_VERSION)
    }
}
