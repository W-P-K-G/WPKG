use std::vec;

use crate::globals;
use crate::utils;

use tracing::*;

use serde::{Deserialize, Serialize};

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

pub async fn install_update(link: &str) -> anyhow::Result<()> {
    // Kill old wpkg
    #[cfg(not(target_os = "windows"))]
    {
        use sysinfo::ProcessExt;
        use sysinfo::SystemExt;
        let mut system = sysinfo::System::new();
        system.refresh_all();
        for p in system.processes_by_name("wpkg") {
            nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(p.pid().to_string().parse()?),
                nix::sys::signal::SIGKILL,
            )?;
        }
    }
    #[cfg(target_os = "windows")]
    {
        utils::run_process("taskkill.exe", vec!["/f", "/im", "wpkg.exe"], true)?;
    }

    let location = utils::get_working_dir()? + r#"/wpkg"#;
    info!("Updating... 2/2");
    #[cfg(target_os = "windows")]
    let suffix = ".exe";

    #[cfg(not(target_os = "windows"))]
    let suffix = "";

    utils::download_from_url(link, &(location.clone() + suffix)).await?;
    utils::run_process(&(location + suffix), vec![""], false)?;
    std::process::exit(0);
}

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

pub async fn check_updates() -> anyhow::Result<()> {
    info!("Checing for updates..");

    let ver: Vec<Versions> = Versions::parse(
        &utils::download_string(
            "https://raw.githubusercontent.com/W-P-K-G/JSONFiles/master/Versions.json",
        )
        .await?,
    )?;

    let nevest_ver = ver[ver.len() - 1].clone();

    if globals::CURRENT_VERSION != nevest_ver.version {
        info!(
            "New version {} founded, current version is {}",
            globals::CURRENT_VERSION,
            nevest_ver.version
        );
        update(&nevest_ver.link).await?
    } else {
        info!("WPKG Up to date!");
    }
    Ok(())
}
