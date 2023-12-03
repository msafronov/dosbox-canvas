use crate::memory_ptr::MemoryPtr;

pub struct AllocatedMemory {
    ptr: MemoryPtr,
    position: usize,
    len: usize,
    capacity: usize,
}

impl AllocatedMemory {
    pub fn new(ptr: MemoryPtr, position: usize, len: usize, capacity: usize) -> Self {
        AllocatedMemory {
            ptr,
            position,
            len,
            capacity,
        }
    }

    pub fn ptr(&self) -> MemoryPtr {
        self.ptr
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}