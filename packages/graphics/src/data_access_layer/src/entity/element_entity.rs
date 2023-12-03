use crate::entity_id::EntityId;

use super::IEntity;

#[repr(C)]
pub struct ElementEntity {
    pub ep_type_id: u8,
    pub ep_enabled: u8,
    pub ep_color_id: u8,
    pub reserved: u8,
    pub ep_x1: u16,
    pub ep_y1: u16,
    pub ep_z1: u8,
    pub ep_x2: u16,
    pub ep_y2: u16,
    pub ep_z2: u8,
    pub ep_border_color_id: u8,
    pub ep_text: [u8; 40],
}

impl ElementEntity {
    pub fn new() -> Self {
        ElementEntity {
            ep_type_id: 0,
            ep_enabled: 0,
            ep_color_id: 0,
            reserved: 0,
            ep_x1: 0,
            ep_y1: 0,
            ep_z1: 0,
            ep_x2: 0,
            ep_y2: 0,
            ep_z2: 0,
            ep_border_color_id: 0,
            ep_text: [0; 40],
        }
    }
}

impl IEntity for ElementEntity {
    fn _id(&self) -> EntityId {
        EntityId::Element
    }

    fn _len(&self) -> usize {
        64
    }
}