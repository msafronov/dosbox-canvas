use crate::entity_id::EntityId;

use super::IEntity;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScreenEntity {
    pub psp_rgba: usize,
}

impl ScreenEntity {
    pub fn new() -> Self {
        ScreenEntity {
            psp_rgba: 0,
        }
    }
}

impl IEntity for ScreenEntity {
    fn _id(&self) -> EntityId {
        EntityId::Screen
    }

    fn _len(&self) -> usize {
        4
    }
}