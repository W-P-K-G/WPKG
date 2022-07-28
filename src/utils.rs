extern crate msgbox;
extern crate systemstat;

use std::process::Command;

use msgbox::*;
use std::time::Duration;
use std::thread;
use systemstat::{saturating_sub_bytes, Platform, System};
use tracing::*;

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
