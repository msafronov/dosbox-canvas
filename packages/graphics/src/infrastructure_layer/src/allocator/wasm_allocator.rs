use core::arch::wasm32;
use errors::Error;

use crate::allocated_memory::AllocatedMemory;
use crate::Allocator;
use crate::IAllocator;
use crate::SizeToWasmBlockMapper;

#[derive(Clone, Copy)]
pub struct WasmAllocator {
    memory_allocator: Allocator,
    mapper: SizeToWasmBlockMapper,
}

impl WasmAllocator {
    pub fn new(memory_allocator: Allocator, mapper: SizeToWasmBlockMapper) -> Self {
        WasmAllocator {
            memory_allocator,
            mapper,
        }
    }
}

impl IAllocator for WasmAllocator {
    fn malloc(&mut self, size: usize) -> AllocatedMemory {
        let allocated_blocks = self.mapper.size_to_wasm_blocks_count(size);

        let previous_block = wasm32::memory_grow(0, allocated_blocks);

        if previous_block == usize::MAX {
            errors::panic(Error::MemoryAllocationWasmMemoryGrowError);
        }

        let wasm_capacity = self.mapper.wasm_blocks_count_to_size(allocated_blocks);

        let allocated_memory = self.memory_allocator.malloc(
            wasm_capacity,
        );

        let wasm_allocated_memory = AllocatedMemory::new(
            allocated_memory.ptr(),
            allocated_memory.position(),
            size,
            wasm_capacity,
        );

        wasm_allocated_memory
    }

    fn free(&self) {
        self.memory_allocator.free()
    }
}