use crate::entity_id::EntityId;

use super::IEntity;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FontEntity {
    pub fp_char: u8,
}

impl FontEntity {
    pub fn new() -> Self {
        FontEntity {
            fp_char: 0,
        }
    }
}

impl IEntity for FontEntity {
    fn _id(&self) -> EntityId {
        EntityId::Font
    }

    fn _len(&self) -> usize {
        1
    }
}