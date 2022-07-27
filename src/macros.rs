#[macro_export]
macro_rules! update_mutex {
    ($mutex: expr, $($new_value:tt)+) => {
        *lock_mutex!($mutex) = $($new_value)+;
    }
}

#[macro_export]
macro_rules! lock_mutex {
    ($mutex: expr) => {
        $mutex.lock().expect("failed to lock mutex")
    };
}
