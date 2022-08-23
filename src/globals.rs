use wpkg_macro::encode;

pub const JSON_ADDRESSES_URL: &str =
    encode!("https://wpkg.medzik.workers.dev/JSONFiles/Addreses.json");
pub const UPDATER_URL: &str = encode!("https://wpkg.medzik.workers.dev/JSONFiles/Versions.json");
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
