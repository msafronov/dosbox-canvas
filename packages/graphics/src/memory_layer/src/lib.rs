#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod registry;
mod registry_record;
mod memory_block_storage;
mod memory_block;
mod memory_block_packet;

use infrastructure_layer::InfrastructureLayer;
use registry::Registry;
use memory_block_storage::MemoryBlockStorage;

pub use memory_block::MemoryBlock;
pub use memory_block_packet::MemoryBlockPacket;

#[derive(Clone, Copy)]
pub struct MemoryLayer {
    infrastructure_level: InfrastructureLayer,
    registry: Registry,
    memory_block_storage: MemoryBlockStorage,
}

impl MemoryLayer {
    pub fn new(infrastructure_level: InfrastructureLayer) -> MemoryLayer {
        let registry = Registry::new();

        let memory_block_storage = MemoryBlockStorage::new(
            infrastructure_level,
            registry,
        );

        MemoryLayer {
            infrastructure_level,
            registry,
            memory_block_storage,
        }
    }

    pub fn init(&self) {
        self.infrastructure_level.init();
        self.registry.init();
    }

    pub fn memory_block_create(
        &mut self,
        memory_block_id: u8,
        packet_len: usize,
        packets_count: usize,
    ) {
        self.memory_block_storage.create(memory_block_id, packet_len, packets_count);
    }

    pub fn memory_block_find_by_id(&self, memory_block_id: u8) -> MemoryBlock {
        self.memory_block_storage.find_by_id(memory_block_id)
    }

    pub fn memory_block_packet_create(&self, memory_block_id: u8) -> MemoryBlockPacket {
        self.memory_block_storage.packet_create(memory_block_id)
    }

    pub fn memory_block_packet_find_by_id(&self, memory_block_id: u8, packet_id: usize) -> MemoryBlockPacket {
        self.memory_block_storage.packet_find_by_id(memory_block_id, packet_id)
    }

    pub fn memory_block_packet_remove_by_id(&self, memory_block_id: u8, packet_id: usize) -> bool {
        self.memory_block_storage.packet_remove_by_id(memory_block_id, packet_id)
    }
}