use crate::ElementTypeId;
use crate::ColorId;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ElementDTO {
    pub type_id: ElementTypeId,
    pub enabled: u8,
    pub color_id: ColorId,
    pub text_locale_id: u8,
    pub x1: u16,
    pub y1: u16,
    pub z1: u8,
    pub x2: u16,
    pub y2: u16,
    pub z2: u8,
    pub border_color_id: ColorId,
    pub text: [u8; 40],
}

impl ElementDTO {
    pub fn new() -> Self {
        ElementDTO {
            type_id: ElementTypeId::NotUsed,
            enabled: 1,
            color_id: ColorId::White,
            text_locale_id: 0,
            x1: 0,
            y1: 0,
            z1: 0,
            x2: 0,
            y2: 0,
            z2: 0,
            border_color_id: ColorId::White,
            text: [0; 40],
        }
    }
}