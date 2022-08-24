use super::prelude::*;
use crate::utils;

pub struct Run;

#[async_trait]
impl Command for Run {
    fn name(&self) -> &'static str {
        encode!("run")
    }

    fn help(&self) -> &'static str {
        encode!("<exe> <args> - Run process")
    }

    fn min_args(&self) -> usize {
        1
    }

    async fn execute(&self, client: &mut Client, args: Vec<&str>) -> anyhow::Result<()> {
        utils::run_process(args[0], args[1..args.len()].to_vec(), false)?;

        Ok(client.send("Done")?)
    }
}
