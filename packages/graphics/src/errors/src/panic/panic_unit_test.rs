use crate::Error;

const ERROR_CODE_POS_IN_MEMORY: u8 = 1;

// it's called during unit tests only. see "panic_default"
pub fn panic(error: Error) {
    let ptr = ERROR_CODE_POS_IN_MEMORY as *mut u8;

    unsafe {
        let existing_error_code = *ptr;

        if existing_error_code != (Error::NoError as u8) {
            return;
        }

        *ptr = error as u8;
    }
}

// it's called during unit tests only. see "panic_default"
pub fn get_first_error() -> Error {
    let ptr = ERROR_CODE_POS_IN_MEMORY as *mut u8;

    let error_code = unsafe {
        let existing_error_code = *ptr;

        *ptr = Error::NoError as u8;

        existing_error_code
    };
    
    Error::from(error_code)
}