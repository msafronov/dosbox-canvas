use crate::screen::Screen;
use crate::ColorId;
use crate::screen_z_index::ScreenZIndex;
use crate::screen_pencil::GeometryToScreenMapper;

#[derive(Clone, Copy)]
pub struct ScreenPencil {
    screen: Screen,
    screen_z_index: ScreenZIndex,
    mapper: GeometryToScreenMapper,
}

impl ScreenPencil {
    pub fn new(
        screen: Screen,
        screen_z_index: ScreenZIndex,
        mapper: GeometryToScreenMapper,
    ) -> Self {
        ScreenPencil {
            screen,
            screen_z_index,
            mapper,
        }
    }

    pub fn draw_point(&mut self, x: u16, y: u16, z: u8, color_id: ColorId) {
        let screen_position = self.mapper.from_geometry_to_screen_pos(x, y);

        self.draw(screen_position, z, color_id);
    }

    pub fn prepare_draw(&mut self) {
        self.screen.prepare_draw();   
    }

    fn draw(&mut self, screen_position: usize, z: u8, color_id: ColorId) {
        // TODO:
        // 1. record data to z-index screen

        // 2. record data to canvas screen
        self.screen.draw(screen_position, color_id);
    }
}