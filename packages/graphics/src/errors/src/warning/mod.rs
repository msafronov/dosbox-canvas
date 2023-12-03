extern "C" {
    pub fn log_warning(arg0: usize);
}

pub fn warning(arg0: usize) {
    unsafe {
        log_warning(arg0);
    }
}