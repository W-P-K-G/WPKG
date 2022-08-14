extern crate msgbox;
extern crate systemstat;

use anyhow::anyhow;
use anyhow::Context;
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

pub async fn download_string(url: &str) -> anyhow::Result<String> {
    Ok(reqwest::get(url).await?.text().await?)
}

pub async fn download_from_url(url: &str, path: &str) -> anyhow::Result<()> {
    let resp = reqwest::get(url).await?;
    let mut out = File::create(path)?;

    let mut content = Cursor::new(resp.bytes().await?);
    io::copy(&mut content, &mut out)?;
    Ok(())
}

/// Show message box
pub fn messagebox(message: String) {
    tokio::spawn(async move { msgbox::create("", &message, IconType::Info) });
}

pub fn run_process_real(exe: &str, args: Vec<&str>, wait: bool) -> anyhow::Result<()> {
    if wait {
        Command::new(exe).args(args).output()?;
    } else {
        Command::new(exe).args(args).spawn()?;
    }
    Ok(())
}

pub fn run_process(exe: &str, args: Vec<&str>, wait: bool) -> anyhow::Result<()> {
    // let mut full_command: Vec<&str> = vec![];

    // #[cfg(target_os = "windows")]
    // {
    //     full_command.push("cmd.exe");
    //     full_command.push("/c");
    // }

    // full_command.push(exe);
    // for arg in args {
    //     full_command.push(arg);
    // }

    // run_process_real(
    //     full_command[0],
    //     full_command[1..full_command.len()].to_vec(),
    //     wait,
    // )?;

    run_process_real(
        exe,
        args,
        wait,
    )?;
    Ok(())
}

pub fn run_process_with_work_dir(
    exe: &str,
    args: Vec<&str>,
    wait: bool,
    current_dir: &str,
) -> anyhow::Result<()> {
    // let mut full_command: Vec<&str> = vec![];

    // #[cfg(target_os = "windows")]
    // {
    //     full_command.push("cmd.exe");
    //     full_command.push("/c");
    // }

    // full_command.push(exe);
    // for arg in args {
    //     full_command.push(arg);
    // }

    // if wait {
    //     Command::new(full_command[0])
    //         .args(full_command[1..full_command.len()].to_vec())
    //         .current_dir(current_dir)
    //         .output()?;
    // } else {
    //     Command::new(full_command[0])
    //         .args(full_command[1..full_command.len()].to_vec())
    //         .current_dir(current_dir)
    //         .spawn()?;
    // }
    if wait {
        Command::new(exe)
            .args(args)
            .current_dir(current_dir)
            .output()?;
    } else {
        Command::new(exe)
            .args(args)
            .current_dir(current_dir)
            .spawn()?;
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
            info!("WPKG dir not exists... Creating it...");
            fs::create_dir(&config_dir)?;
        }

        Ok(config_dir)
    }
}

pub fn screenshot() -> anyhow::Result<String> {
    info!("Taking screenshot...");
    let screens = Screen::all().ok_or(anyhow!("Can't take screenshot!"))?;

    if screens.is_empty() {
        return Err(anyhow!("Screen is empty"));
    }

    let image = screens
        .get(0)
        .context("Could not find screens")?
        .capture()
        .context("Image is empty")?;
    let buffer = image.buffer();

    // Save the image.
    let mut rng = rand::thread_rng();
    let save_path = format!("{}/image{}.png", get_working_dir()?, rng.gen::<i32>());
    fs::write(&save_path, &buffer)?;

    Ok(save_path)
}

pub async fn screenshot_url() -> anyhow::Result<String> {
    let path = screenshot()?;
    let info = ImgurClient::new("3e3ce0d7ac14d56")
        .upload_image(&path)
        .await?;

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
