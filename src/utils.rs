// use with `map_err`
pub fn inspect<T>(message: &str) -> impl FnOnce(T) -> T + '_ {
    move |err| {
        eprintln!("Error: {}", message);
        err
    }
}
