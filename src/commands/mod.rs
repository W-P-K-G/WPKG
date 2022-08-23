mod disconnect;
mod help;
mod ping;
mod run;
mod screenshot;
mod stat;
mod version;

use std::any::Any;

use async_trait::async_trait;

use crate::client::Client;

use self::{
    disconnect::Disconnect, help::Help, ping::Ping, run::Run, screenshot::Screenshot, stat::Stat,
    version::Version,
};

#[async_trait]
pub trait Command: Any + Send + Sync {
    fn name(&self) -> &'static str;

    fn help(&self) -> &'static str;

    fn min_args(&self) -> usize;

    async fn execute(&self, client: &mut Client, args: Vec<&str>) -> anyhow::Result<()>;
}

#[derive(Default)]
pub struct CommandsManager {
    pub commands: Vec<Box<dyn Command>>,
}

impl CommandsManager {
    pub fn new() -> Self {
        Self {
            // Don't forget to add commands to the Vec!
            commands: vec![
                Box::new(Stat),
                Box::new(Run),
                Box::new(Screenshot),
                Box::new(Disconnect),
                Box::new(Ping),
                Box::new(Version),
                Box::new(Help),
            ],
        }
    }
}

mod prelude {
    pub use super::*;

    pub use crate::client::Client;
    pub(crate) use crate::encode;
    pub use async_trait::async_trait;
    pub extern crate anyhow;
}
