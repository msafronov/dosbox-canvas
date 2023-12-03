pub struct MemoryBlockPacket {
    position: usize,
}

impl MemoryBlockPacket {
    pub fn new(position: usize) -> Self {
        MemoryBlockPacket {
            position,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }
}