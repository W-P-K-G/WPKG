extern crate msgbox;

use msgbox::*;
use std::thread;

pub fn messagebox(message: String) {
    thread::spawn(move || msgbox::create("", &message, IconType::Info));
}
