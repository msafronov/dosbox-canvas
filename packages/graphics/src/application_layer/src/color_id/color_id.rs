#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ColorId {
    NotUsed = 0,
    White,
    Black,
    Gray,
    Blue,
    Teal,
    Green,
    GreenAcid,
    Yellow,
    Red,
    Maroon,
    Purple,
}

impl ColorId {
    pub fn from(color_id: u8) -> ColorId {
        match color_id {
            1 => ColorId::White,
            2 => ColorId::Black,
            3 => ColorId::Gray,
            4 => ColorId::Blue,
            5 => ColorId::Teal,
            6 => ColorId::Green,
            7 => ColorId::GreenAcid,
            8 => ColorId::Yellow,
            9 => ColorId::Red,
            10 => ColorId::Maroon,
            11 => ColorId::Purple,
            _ => ColorId::NotUsed
        }
    }
}