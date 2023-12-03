use memory_layer::MemoryLayer;

use crate::entity::IEntity;

use super::IRepository;

#[derive(Clone, Copy)]
pub struct Repository {
    memory_layer: MemoryLayer,
    position_for_raw_ops: usize,
}

impl Repository {
    pub fn new(memory_layer: MemoryLayer) -> Self {
        Repository {
            memory_layer,
            position_for_raw_ops: 0,
        }
    }

    pub fn insert<T: IEntity>(&self, entity: T) {
        let memory_block_id = entity._id() as u8;
        let packet = self.memory_layer.memory_block_packet_create(memory_block_id);
        let ptr = packet.position() as *mut T;

        if ptr.is_null() {
            return;
        }

        unsafe {
            *ptr = entity;
        }
    }

    pub fn prepare_raw<T: IEntity>(&mut self, entity: T) {
        let memory_block_id = entity._id() as u8;
        let memory_block = self.memory_layer.memory_block_find_by_id(memory_block_id);

        self.position_for_raw_ops = memory_block.position();
    }

    // dangerous writing direct to the memory when better performance is needed
    // need to call "prepare_raw" before
    pub fn update_raw<T: IEntity>(&self, position_offset: usize, entity: T) {
        let ptr = (self.position_for_raw_ops + position_offset) as *mut T;

        if ptr.is_null() {
            return;
        }

        unsafe {
            *ptr = entity;
        }
    }
}

impl IRepository for Repository {}