extern crate msgbox;

use std::thread;
use msgbox::*;

pub fn messagebox(message: String)
{
    thread::spawn(move || msgbox::create("Hyhyhy",&message, IconType::Info).unwrap());
}