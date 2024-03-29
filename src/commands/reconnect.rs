use super::prelude::*;

pub struct Reconnect;

#[async_trait]
impl Command for Reconnect {
    fn name(&self) -> String {
        crypto!("reconnect")
    }

    fn help(&self) -> String {
        crypto!("<ip> <port> - Reconnecting to another ServerD")
    }

    fn min_args(&self) -> usize {
        2
    }

    async fn execute(&self, client: &mut Client, args: Vec<&str>) -> anyhow::Result<()> {
        match Client::new(&format!("{}:{}", args[0], args[1])) {
            Ok(mut new_client) => {
                info!(
                    "{} {}: {}",
                    crypto!("Reconnected succesfully to"),
                    args[0],
                    args[1]
                );

                client.send(crypto!(
                    "Succesfully reconnected client... disconnecting..."
                ))?;

                //not send_command
                client.send("/disconnect")?;

                client.reconnecting = true;

                client.close()?;

                new_client.run().await?;
            },
            Err(_e) => {
                let msg = crypto!("Error reconnecting to server");

                error!(msg);
                client.send(&msg)?;
            },
        }

        Ok(())
    }
}
