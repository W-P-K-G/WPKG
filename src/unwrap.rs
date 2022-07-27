pub trait CustomUnwrap {
    fn unwrap_log(self);
    fn expect_log(self, msg: &str);
}

impl<T, E> CustomUnwrap for Result<T, E>
where
    E: ToString,
{
    fn unwrap_log(self) {
        if let Err(e) = self {
            tracing::error!("{e}", e = e.to_string())
        }
    }

    fn expect_log(self, msg: &str) {
        if let Err(e) = self {
            tracing::error!("{msg}: {e}", e = e.to_string())
        }
    }
}
