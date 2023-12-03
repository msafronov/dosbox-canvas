#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

use presentation_layer::PresentationLayer;
use application_layer::ApplicationLayer;
use data_access_layer::DataAccessLayer;
use memory_layer::MemoryLayer;
use infrastructure_layer::InfrastructureLayer;

pub struct Main {}

impl Main {
    fn get_application(&self) -> PresentationLayer {
        PresentationLayer::new(
            ApplicationLayer::new(
                DataAccessLayer::new(
                    MemoryLayer::new(
                        InfrastructureLayer::new(),
                    ),
                ),
            ),
        )
    }

    // public api

    #[no_mangle]
    pub fn init(&self) {
        self.get_application().init();
    }

    #[no_mangle]
    pub fn run(&self) {
        self.get_application().run();
    }

    #[no_mangle]
    pub fn screen_width(&self) -> i32 {
        self.get_application().screen_width()
    }
    
    #[no_mangle]
    pub fn screen_height(&self) -> i32 {
        self.get_application().screen_height()
    }
}
