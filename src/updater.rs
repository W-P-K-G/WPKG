use crate::globals;
use crate::globals::UPDATER_URL;
use crate::utils;

use serde::{Deserialize, Serialize};
use tracing::*;

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
    // Kill old wpkg
    #[cfg(target_os = "windows")]
    {
        utils::run_process("taskkill.exe", vec!["/f", "/im", "wpkg.exe"], true)?;
    }

    let location = utils::get_working_dir()? + r#"/wpkg"#;
    info!("Updating... 2/2");
    #[cfg(target_os = "windows")]
    let suffix = ".exe";

    utils::download_from_url(link, &(location.clone() + suffix)).await?;
    utils::run_process(&(location + suffix), vec![""], false)?;
    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub async fn install_update(_link: &str) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn update(link: &str) -> anyhow::Result<()> {
    info!("Updating... 1/2");
    let target = utils::get_working_dir()? + r#"/update"#;

    #[cfg(target_os = "windows")]
    let suffix = ".exe";

    #[cfg(not(target_os = "windows"))]
    let suffix = "";

    utils::download_from_url(link, &(target.clone() + suffix)).await?;
    utils::run_process(&(target + suffix), vec!["--update", link], false)?;
    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub async fn update(_link: &str) -> anyhow::Result<()> {
    Ok(())
}

pub async fn check_updates() -> anyhow::Result<()> {
    info!("Checking for updates..");

    let ver: Vec<Versions> =
        Versions::parse(&utils::download_string(&String::from_utf8(UPDATER_URL.to_vec())?).await?)?;

    let newest_ver = ver[ver.len() - 1].clone();

    if globals::CURRENT_VERSION != newest_ver.version {
        info!(
            "New version {} founded, current version is {}",
            newest_ver.version,
            globals::CURRENT_VERSION
        );
        update(&newest_ver.link).await?
    } else {
        info!("WPKG Up to date!");
    }
    Ok(())
}
