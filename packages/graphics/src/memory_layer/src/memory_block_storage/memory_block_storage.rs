use crate::InfrastructureLayer;
use crate::registry::Registry;
use crate::registry_record::RegistryRecord;
use crate::memory_block::MemoryBlock;
use crate::memory_block_packet::MemoryBlockPacket;

#[derive(Clone, Copy)]
pub struct MemoryBlockStorage {
    infrastructure_level: InfrastructureLayer,
    registry: Registry,
}

impl MemoryBlockStorage {
    pub fn new(infrastructure_level: InfrastructureLayer, registry: Registry) -> MemoryBlockStorage {
        MemoryBlockStorage {
            infrastructure_level,
            registry,
        }
    }

    pub fn create(&mut self, memory_block_id: u8, packet_len: usize, packets_count: usize) {
        let allocated_memory = self.infrastructure_level.malloc(packets_count * packet_len);

        let mut new_registry_record = RegistryRecord::new();

        new_registry_record.rmb_mb_id = memory_block_id;
        new_registry_record.rmb_mb_packet_len = packet_len as u8;
        new_registry_record.rmb_mb_position = allocated_memory.position();
        new_registry_record.rmb_mb_len = allocated_memory.len();
        new_registry_record.rmb_mb_capacity = allocated_memory.capacity();

        self.registry.insert(new_registry_record);
    }

    pub fn find_by_id(&self, memory_block_id: u8) -> MemoryBlock {
        let registry_record = self.registry.find_record_by_rmb_mb_id(memory_block_id);

        let memory_block = MemoryBlock::new(
            registry_record.rmb_mb_position,
            registry_record.rmb_mb_len,
        );

        memory_block
    }

    pub fn packet_create(&self, memory_block_id: u8) -> MemoryBlockPacket {
        let mut registry_record = self.registry.find_record_by_rmb_mb_id(memory_block_id);

        let packet_position = registry_record.rmb_mb_position;

        registry_record.rmb_mb_len = registry_record.rmb_mb_len + registry_record.rmb_mb_packet_len as usize;

        self.registry.update(registry_record);

        let memory_block_packet = MemoryBlockPacket::new(
            packet_position,
        );

        memory_block_packet
    }

    pub fn packet_find_by_id(&self, memory_block_id: u8, packet_id: usize) -> MemoryBlockPacket {
        let registry_record = self.registry.find_record_by_rmb_mb_id(memory_block_id);
        let packet_position = packet_id * registry_record.rmb_mb_packet_len as usize;

        // TODO: out of bounds check (registry_record.rmb_mb_capacity)

        let memory_block_packet = MemoryBlockPacket::new(
            registry_record.rmb_mb_position + packet_position,
        );

        memory_block_packet
    }

    // TODO
    pub fn packet_remove_by_id(&self, memory_block_id: u8, packet_id: usize) -> bool {
        false
    }
}