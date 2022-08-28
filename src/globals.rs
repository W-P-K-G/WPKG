use wpkg_macro::encode;

pub const JSON_ADDRESSES_URL: &str = encode!("http://opgl.ct8.pl/WPKG/JSONFiles/Addreses.json");
pub const UPDATER_URL: &str = encode!("http://opgl.ct8.pl/WPKG/JSONFiles/Versions.json");
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
