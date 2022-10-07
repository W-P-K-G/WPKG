#[macro_export]
macro_rules! crypto {
    ($input:expr) => {
        wpkg_crypto::decode(wpkg_macro::encode!($input))
    };
}

#[macro_export]
macro_rules! info_crypt {
    ($msg:expr) => {
        tracing::info!("{}", $crate::crypto!($msg))
    };
}

#[macro_export]
macro_rules! warn_crypt {
    ($msg:expr) => {
        tracing::warn!("{}", $crate::crypto!($msg))
    };
}

#[macro_export]
macro_rules! error_crypt {
    ($msg:expr) => {
        tracing::error!("{}", $crate::crypto!($msg))
    };
}
