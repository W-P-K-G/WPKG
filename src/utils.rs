extern crate msgbox;

use msgbox::*;

/// Show message box
pub fn messagebox(message: String) {
    tokio::spawn(async move { msgbox::create("", &message, IconType::Info) });
}
