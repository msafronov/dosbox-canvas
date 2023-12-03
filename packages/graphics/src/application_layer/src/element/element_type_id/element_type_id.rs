#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ElementTypeId {
    NotUsed = 0,
    Panel,
    Text,
}

impl ElementTypeId {
    pub fn from(element_type_id: u8) -> ElementTypeId {
        match element_type_id {
            1 => ElementTypeId::Panel,
            2 => ElementTypeId::Text,
            _ => ElementTypeId::NotUsed
        }
    }
}