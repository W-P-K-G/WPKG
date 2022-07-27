#[macro_export]
macro_rules! update_mutex {
    ($mutex: expr, $new_value: expr) => {
        let mut value = lock_mutex!($mutex);
        *value = $new_value;
    }
}

#[macro_export]
macro_rules! lock_mutex {
    ($mutex: expr) => {
        $mutex.lock().unwrap()
    }
}
