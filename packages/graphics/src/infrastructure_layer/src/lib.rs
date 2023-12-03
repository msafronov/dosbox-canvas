#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod config;
mod allocator;
mod allocated_memory;
mod memory_ptr;

use allocated_memory::AllocatedMemory;
use allocator::IAllocator;
use allocator::Allocator;
use allocator::WasmAllocator;

pub use memory_ptr::MemoryPtr;
pub use allocator::SizeToWasmBlockMapper;

#[derive(Clone, Copy)]
pub struct InfrastructureLayer {
    allocator: WasmAllocator,
}

impl InfrastructureLayer {
    pub fn new() -> Self {
        let allocator = WasmAllocator::new(
            Allocator::new(
                config::ALLOCATOR_MEMORY_INITIAL_POSITION,
            ),
            SizeToWasmBlockMapper::new(
                config::WASM_BLOCK_SIZE,
            ),
        );

        InfrastructureLayer {
            allocator,
        }
    }

    pub fn init(&self) {
        //
    }

    pub fn malloc(&mut self, size: usize) -> AllocatedMemory {
        self.allocator.malloc(size)
    }
}