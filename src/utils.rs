extern crate msgbox;
extern crate systemstat;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::os::unix::prelude::PermissionsExt;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::vec;

use imgurs::ImgurClient;
use msgbox::*;
use rand::prelude::*;
use screenshots::Screen;
use sysinfo::ProcessExt;
use sysinfo::SystemExt;
use systemstat::{saturating_sub_bytes, Platform, System};
use tracing::*;

use crate::globals;
use crate::versions::Versions;

pub struct Utils;

impl Utils {
    pub async fn update(link: &str) -> anyhow::Result<()>{
        
        
        // Kill old wpkg
        #[cfg(not(target_os="windows"))]
        {
            let mut system = sysinfo::System::new();
            system.refresh_all();
            for p in system.processes_by_name("wpkg") {
                nix::sys::signal::kill(nix::unistd::Pid::from_raw(p.pid().to_string().parse()?), nix::sys::signal::SIGKILL).expect("s");
            }
        }

        let location = Self::get_working_dir()?+r#"/wpkg"#;
        info!("Updating");
        #[cfg(target_os="windows")]
        let suffix = ".exe";

        #[cfg(not(target_os="windows"))]
        let suffix = "";
        
        Self::download_from_url(link, &(location.clone()+suffix)).await?;
        Self::run_process(&(location+suffix), vec![""], false)?;
        panic!("Kurwa zjebało się");
    }
    pub async fn check_updates() -> anyhow::Result<()>{
        info!("checking");
        let ver: Vec<Versions> = Versions::parse(
            &Self::download_string(
                "https://raw.githubusercontent.com/W-P-K-G/JSONFiles/master/Versions.json").await?)?;
        let nevest_ver = ver[ver.len()-1].clone();
        if globals::CURRENT_VERSION != nevest_ver.version{
            let target = Self::get_working_dir()?+r#"/update"#;
            #[cfg(target_os="windows")]
            let suffix = ".exe";
    
            #[cfg(not(target_os="windows"))]
            let suffix = "";
            Self::download_from_url(&nevest_ver.link, &(target.clone()+suffix)).await?;
            Self::run_process(&(target+suffix), vec!["--update", &nevest_ver.link], false)?;
            panic!();
        } else {
            info!("WPKG Up to date!");
        }
        Ok(())
    }
    pub async fn download_string(url: &str) -> anyhow::Result<String>{
        Ok(reqwest::get(url).await?.text().await?)
    }
    pub async fn download_from_url(url: &str, path: &str) -> anyhow::Result<()>{
        let resp = reqwest::get(url).await?;
        let mut out = File::create(path)?;
        
        #[cfg(not(target="windows"))]{
            let mut permissions = out.metadata()?.permissions();
            permissions.set_mode(0o777);
            out.set_permissions(permissions)?;
        }

        let mut content =  Cursor::new(resp.bytes().await?);
        io::copy(&mut content, &mut out)?;
        Ok(())
    }
    /// Show message box
    pub fn messagebox(message: String) {
        tokio::spawn(async move { msgbox::create("", &message, IconType::Info) });
    }

    pub fn run_process(exe: &str, args: Vec<&str>, wait: bool) -> anyhow::Result<()> {
        if wait {
            Command::new(exe).args(args).output()?;
        } else {
            Command::new(exe).args(args).spawn()?;
        }
        Ok(())
    }

    pub fn run_process_with_work_dir(exe: &str, args: &str, wait: bool, currentdir: &str) -> anyhow::Result<()> {
        if wait {
            Command::new(exe)
                .args(&[args])
                .current_dir(currentdir)
                .output()
                ?;
        } else {
            Command::new(exe)
                .args(&[args])
                .current_dir(currentdir)
                .spawn()
                ?;
        }
        Ok(())
    }

    pub fn get_working_dir() -> anyhow::Result<String> {
        #[cfg(not(target_os = "windows"))]
        return Ok(env::current_dir()?.display().to_string());

        #[cfg(target_os = "windows")]
        {
            use platform_dirs::AppDirs;

            let app_dirs = AppDirs::new(Some("WPKG"), true)?;
            let config_dir = app_dirs.config_dir.display().to_string();

            if !Path::new(&config_dir).exists() {
                info!("WPKG dir not exists... Creating it...");
                fs::create_dir(&config_dir)?;
            }

            return Ok(config_dir);
        }
    }

    pub fn screenshot() -> anyhow::Result<String> {
        info!("Taking screenshot...");
        let screens = Screen::all();

        let image = screens[0].capture().unwrap();
        let buffer = image.buffer();

        // Save the image.
        let mut rng = rand::thread_rng();
        let savepath = format!("{}/image{}.png", Utils::get_working_dir()?, rng.gen::<i32>());
        fs::write(&savepath, &buffer)?;

        Ok(savepath)
    }

    pub async fn screenshot_url() -> anyhow::Result<String> {
        let path = Utils::screenshot()?;
        let info = ImgurClient::new("3e3ce0d7ac14d56")
            .upload_image(&path)
            .await
            ?;

        Ok(info.data.link)
    }

    pub fn stat() -> String {
        // get system status
        let sys = System::new();

        // init variables
        let mut cpu_usage = 0.;
        let mut memory_free = 0;
        let mut memory_total = 0;
        let mut swap_free = 0;
        let mut swap_total = 0;

        // get cpu usage
        match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                cpu_usage = cpu.user * 100.0;
            }
            Err(x) => {
                error!("CPU load: error: {}", x);
            }
        }

        // get memory stats
        match sys.memory() {
            Ok(mem) => {
                memory_free = saturating_sub_bytes(mem.total, mem.free).as_u64();
                memory_total = mem.total.as_u64();
            }
            Err(x) => {
                error!("\nMemory: error: {}", x);
            }
        }

        // get swap stats
        match sys.swap() {
            Ok(swap) => {
                swap_free = saturating_sub_bytes(swap.total, swap.free).as_u64();
                swap_total = swap.total.as_u64();
            }
            Err(x) => {
                error!("\nMemory: error: {}", x);
            }
        }

        // return stats
        format!(
            "{} {} {} {} {}",
            cpu_usage, memory_free, memory_total, swap_free, swap_total
        )
    }
}
