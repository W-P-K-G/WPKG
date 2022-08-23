use super::prelude::*;

pub struct Disconnect;

#[async_trait]
impl Command for Disconnect {
    fn name(&self) -> &'static str {
        encode!("disconnect")
    }

    fn help(&self) -> &'static str {
        encode!("disconnect from the server")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send("Done")?;

        client.send_command("/disconnect")?;
        client.close()?;

        Ok(())
    }
}
