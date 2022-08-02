#[macro_export]
macro_rules! update_mutex {
    ($mutex: expr, $($new_value:tt)+) => {
        *$mutex.lock().unwrap() = $($new_value)+;
    };
}
