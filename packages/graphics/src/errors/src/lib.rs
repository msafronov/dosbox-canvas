#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod panic;
mod warning;

pub use panic::panic;
pub use panic::get_first_error;
pub use warning::warning;

#[derive(Debug)]
#[repr(u8)]
pub enum Error {
    NoError = 0,

    // system errors

    MemoryAllocationInvalidPointer,
    MemoryAllocationWasmMemoryGrowError,
    MemoryBlockInvalidPacketCapacity,
    MemoryBlockPacketOutOfBounds,
    MemoryBlockNotFound,
    MemoryBlockInvalidPacket,
    MemoryBlockPacketNotFound,

    // domain errors

    CoordinateSystemInvalidOrigin = 100,
    CoordinateSystemInvalidAbscissa,
    CoordinateSystemInvalidOrdinate,
    CoordinateSystemInvalidApplicate,
    RectInvalidNegativeWidth,
    RectInvalidNegativeHeight,
    FlatRectInvalid,
}

impl Error {
    fn from(error_code: u8) -> Error {
        match error_code {
            1 => Error::MemoryAllocationInvalidPointer,
            2 => Error::MemoryAllocationWasmMemoryGrowError,
            3 => Error::MemoryBlockInvalidPacketCapacity,
            4 => Error::MemoryBlockPacketOutOfBounds,
            5 => Error::MemoryBlockNotFound,
            6 => Error::MemoryBlockInvalidPacket,
            7 => Error::MemoryBlockPacketNotFound,

            100 => Error::CoordinateSystemInvalidOrigin,
            101 => Error::CoordinateSystemInvalidAbscissa,
            102 => Error::CoordinateSystemInvalidOrdinate,
            103 => Error::CoordinateSystemInvalidApplicate,
            104 => Error::RectInvalidNegativeWidth,
            105 => Error::RectInvalidNegativeHeight,
            106 => Error::FlatRectInvalid,

            _ => Error::NoError
        }
    }
}