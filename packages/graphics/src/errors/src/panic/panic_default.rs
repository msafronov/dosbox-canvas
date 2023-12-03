use crate::Error;

extern "C" {
    pub fn log_error(error_code: u8);
}

pub fn panic(error: Error) {
    unsafe {
        log_error(error as u8);
    }

    core::arch::wasm32::unreachable();
}

pub fn get_first_error() -> Error {
    Error::NoError
}