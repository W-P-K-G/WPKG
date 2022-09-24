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

    /*
        Use 
        scess(message) when command executed succesfully
        error(message) when command executed failed.
        This added error code which will be recognized by WPKG CLI
     */
    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {

        Ok(client.send(scess(crypto!("message")))?)
    }
}
