use super::prelude::*;

pub struct Disconnect;

#[async_trait]
impl Command for Disconnect {
    fn name(&self) -> String {
        crypto!("disconnect")
    }

    fn help(&self) -> String {
        crypto!("disconnect from the server")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        client.send_command("/disconnect")?;
        client.close()
    }
}
