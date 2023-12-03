pub struct MemoryBlock {
    position: usize,
    len: usize,
}

impl MemoryBlock {
    pub fn new(position: usize, len: usize) -> Self {
        MemoryBlock {
            position,
            len,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn len(&self) -> usize {
        self.len
    }
}