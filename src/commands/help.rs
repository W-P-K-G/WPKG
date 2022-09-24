use wpkg_crypto::decode;

use super::prelude::*;
use crate::client::COMMANDS;

pub struct Help;

#[async_trait]
impl Command for Help {
    fn name(&self) -> String {
        crypto!("help")
    }

    fn help(&self) -> String {
        crypto!("Help menu")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        let mut msg = Vec::new();

        for command in COMMANDS.commands.iter() {
            msg.push(format!(
                "`{}` {}",
                decode(&command.name()),
                decode(&command.help())
            ));
        }

        client.send(msg.join("\n"))
    }
}
