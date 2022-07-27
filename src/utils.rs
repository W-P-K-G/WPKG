extern crate msgbox;

use msgbox::*;

pub fn messagebox(message: String) {
    tokio::spawn(async move {
        msgbox::create("", &message, IconType::Info)
    });
}
