pub mod panic_default;
pub mod panic_unit_test;

#[cfg(feature = "unit_test")]
pub use panic_unit_test::*;

#[cfg(not(feature = "unit_test"))]
pub use panic_default::*;