use crate::entity_id::EntityId;

use super::IEntity;

#[repr(C)]
pub struct ScreenZIndexEntity {
    pub zip_index_0: u8,
    pub zip_index_1: u8,
    pub zip_index_2: u8,
    pub zip_index_3: u8,
}

impl ScreenZIndexEntity {
    pub fn new() -> Self {
        ScreenZIndexEntity {
            zip_index_0: 0,
            zip_index_1: 0,
            zip_index_2: 0,
            zip_index_3: 0,
        }
    }
}

impl IEntity for ScreenZIndexEntity {
    fn _id(&self) -> EntityId {
        EntityId::ScreenZIndex
    }

    fn _len(&self) -> usize {
        4
    }
}