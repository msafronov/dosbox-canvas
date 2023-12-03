#[derive(Debug)]
#[repr(u8)]
pub enum EntityId {
    NotUsed = 0,
    Registry,
    Font,
    Screen,
    ScreenZIndex,
    Element,
}

impl EntityId {
    pub fn from(entity_id: u8) -> EntityId {
        match entity_id {
            1 => EntityId::Registry,
            2 => EntityId::Font,
            3 => EntityId::Screen,
            4 => EntityId::ScreenZIndex,
            5 => EntityId::Element,
            _ => EntityId::NotUsed
        }
    }
}