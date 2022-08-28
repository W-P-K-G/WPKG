use super::prelude::*;
use crate::updater::{self};

pub struct DevUpdate;

#[async_trait]
impl Command for DevUpdate {
    fn name(&self) -> &'static str {
        encode!("dev-update")
    }

    fn help(&self) -> &'static str {
        encode!("<url> - Downloading and installing custom version")
    }

    fn min_args(&self) -> usize {
        1
    }

    async fn execute(&self, client: &mut Client, args: Vec<&str>) -> anyhow::Result<()> {
        client.send(&crypto!("Installing developer build... Disconnecting..."))?;

        client.send("/disconnect")?;

        if let Err(err) = updater::update(args[0]).await {
            error!("{}: {}", crypto!("Updating failed"), err)
        }
        Ok(())
    }
}
