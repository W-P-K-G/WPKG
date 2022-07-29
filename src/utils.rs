extern crate msgbox;
extern crate systemstat;

use std::process::Command;
use std::thread;
use std::time::Duration;
use std::env;
use std::io::ErrorKind::WouldBlock;
use std::fs::File;

use rand::prelude::*;
use scrap::{Capturer, Display};
use msgbox::*;
use systemstat::{saturating_sub_bytes, Platform, System};
use tracing::*;

pub struct Utils;

impl Utils {
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

    pub fn get_working_dir() -> String
    {
        env::current_dir().unwrap().display().to_string()
    }

    pub fn screenshot() -> String
    {
        info!("Taking screenshot...");

        let one_second = Duration::new(1, 0);
        let one_frame = one_second / 60; // czemu tu jest dzielone na 60?
    
        let display = Display::primary().expect("Couldn't find primary display.");
        let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
        let (w, h) = (capturer.width(), capturer.height());
    
        loop {
            // Wait until there's a frame.
    
            let buffer = match capturer.frame() {
                Ok(buffer) => buffer,
                Err(error) => {
                    if error.kind() == WouldBlock {
                        // Keep spinning.
                        thread::sleep(one_frame);
                        continue;
                    } else {
                        panic!("Error: {}", error);
                    }
                }
            };
    
            info!("Captured! Saving...");
    
            // Flip the ARGB image into a BGRA image.
    
            let mut bitflipped = Vec::with_capacity(w * h * 4);
            let stride = buffer.len() / h;
    
            for y in 0..h {
                for x in 0..w {
                    let i = stride * y + 4 * x;
                    bitflipped.extend_from_slice(&[
                        buffer[i + 2],
                        buffer[i + 1],
                        buffer[i],
                        255,
                    ]);
                }
            }
    
            // Save the image.
            let mut rng = rand::thread_rng();
            let savepath = format!("{}/image{}.png",Utils::get_working_dir(), rng.gen::<i32>());
    
            repng::encode(
                File::create(&savepath).unwrap(),
                w as u32,
                h as u32,
                &bitflipped,
            ).unwrap();

            return savepath;
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
}
