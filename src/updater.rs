use serde::{Deserialize, Serialize};
use tracing::*;
use wpkg_crypto::decode;

use crate::{crypto, globals, globals::UPDATER_URL, info_crypt, utils};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Versions {
    pub version: String,
    pub link: String,
}
impl Versions {
    pub fn parse(data: &str) -> serde_json::Result<Vec<Self>> {
        serde_json::from_str(data)
    }
}

#[cfg(target_os = "windows")]
pub async fn install_update(link: &str) -> anyhow::Result<()> {
    let location = format!("{}/wpkg", utils::get_working_dir()?);

    info_crypt!("Updating... 2/2");

    let suffix = crypto!(".exe");

    utils::download_from_url(link, &format!("{location}{suffix}")).await?;
    utils::run_process(&format!("{location}{suffix}"), vec![""], false)?;

    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub async fn install_update(_link: &str) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn update(link: &str) -> anyhow::Result<()> {
    info_crypt!("Updating... 1/2");
    let target = format!("{}/{}}", utils::get_working_dir()?, crypto!("update"));

    let suffix = crypto!(".exe");

    utils::download_from_url(link, &format!("{target}{suffix}")).await?;
    utils::run_process(
        &format!("{target}{suffix}"),
        vec![&crypto!("--update"), link],
        false,
    )?;
    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub async fn update(_link: &str) -> anyhow::Result<()> {
    Ok(())
}

pub async fn check_updates() -> anyhow::Result<(bool, String, String)> {
    info_crypt!("Checking for updates..");

    let uri = decode(UPDATER_URL);

    let ver: Vec<Versions> = Versions::parse(&utils::download_string(&uri).await?)?;

    let newest_ver = ver[ver.len() - 1].clone();

    if globals::CURRENT_VERSION != newest_ver.version {
        info!(
            "{} {}, {} {}",
            crypto!("New version found"),
            newest_ver.version,
            crypto!("Current version is"),
            globals::CURRENT_VERSION
        );

        Ok((false, newest_ver.version, newest_ver.link))
    } else {
        info_crypt!("WPKG Up to date!");
        Ok((true, globals::CURRENT_VERSION.to_string(), "".to_string()))
    }
}
