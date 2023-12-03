use crate::allocated_memory::AllocatedMemory;

pub trait IAllocator {
    fn malloc(&mut self, size: usize) -> AllocatedMemory;
    fn free(&self);
}