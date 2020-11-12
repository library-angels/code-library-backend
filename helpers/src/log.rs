pub fn log_error_and_panic(e: impl std::fmt::Display) {
    log::error!("{}", e);
    panic!("{}", e);
}
