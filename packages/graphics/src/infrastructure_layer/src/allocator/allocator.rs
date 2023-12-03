use errors::Error;
use crate::memory_ptr::MemoryPtr;
use crate::allocated_memory::AllocatedMemory;

use super::IAllocator;

extern "C" {
    static __data_end: usize;
    static __heap_base: usize;
}

#[derive(Clone, Copy)]
pub struct Allocator {
    position: usize,
}

impl Allocator {
    pub fn new(position: usize) -> Self {
        Allocator {
            position,
        }
    }
}

impl IAllocator for Allocator {
    fn malloc(&mut self, size: usize) -> AllocatedMemory {
        let previous_position = self.position;
        let ptr = previous_position as MemoryPtr;
        let capacity = size;

        if ptr.is_null() {
            errors::panic(Error::MemoryAllocationInvalidPointer);
        }

        // bump allocator
        self.position += size;

        AllocatedMemory::new(
            ptr,
            previous_position,
            size,
            capacity,
        )
    }

    fn free(&self) {
        todo!()
    }
}