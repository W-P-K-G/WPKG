extern crate msgbox;
use std::process::Command;

use msgbox::*;

/// Show message box
pub fn messagebox(message: String) {
    tokio::spawn(async move { msgbox::create("", &message, IconType::Info) });
}


pub fn run_process(exe: &str,args: &str,wait: bool)
{
    if wait {
        Command::new(exe).args(&[args]).output().unwrap();
    }
    else {
        Command::new(exe).args(&[args]).spawn().unwrap();
    }
}