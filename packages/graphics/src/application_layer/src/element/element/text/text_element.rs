use crate::element::element::ielement::IElement;
use crate::element::element_dto::ElementDTO;
use crate::screen_pencil::ScreenPencil;
use crate::PERFECT_DOS_VGA_437_8x16_FONT;

pub struct TextElement {
    element_dto: ElementDTO,
    screen_pencil: ScreenPencil,
    font_width: u8,
    font_height: u8,
}

impl TextElement {
    pub fn new(
        element_dto: ElementDTO,
        screen_pencil: ScreenPencil,
        font_width: u8,
        font_height: u8,
    ) -> Self {
        TextElement {
            element_dto,
            screen_pencil,
            font_width,
            font_height,
        }
    }
}

impl IElement for TextElement {
    fn render(&mut self) {
        self.screen_pencil.prepare_draw();

        let mut char_counter: i8 = -1;

        for char in self.element_dto.text {
            char_counter = char_counter + 1;

            // char id=0 is not available in the app
            if char == 0 {
                continue;
            }

            let char_lines = PERFECT_DOS_VGA_437_8x16_FONT[char as usize];

            let x1 = self.element_dto.x1 + (char_counter as u8 * self.font_width) as u16;

            let mut row: i16 = -1;

            for char_line_as_u8 in char_lines {
                let mut col: u8 = 0;
                let mut bitwise_counter = self.font_width;

                row = row + 1;

                while bitwise_counter > 0 {
                    col = col + 1;
                    bitwise_counter = bitwise_counter - 1;

                    let need_to_draw_char_pixel = (char_line_as_u8 >> bitwise_counter) & 0x01 == 1;

                    if !need_to_draw_char_pixel {
                        continue;
                    }

                    self.screen_pencil.draw_point(
                        x1 + col as u16,
                        self.element_dto.y1 + row as u16,
                        self.element_dto.z1,
                        self.element_dto.color_id,
                    );
                }
            }
        }
    }
}