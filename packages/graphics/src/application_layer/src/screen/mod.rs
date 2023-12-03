use crate::ColorId;
use crate::Repository;
use crate::ScreenEntity;
use crate::color;

#[derive(Clone, Copy)]
pub struct Screen {
    repository: Repository,
    width: i32,
    height: i32,
}

impl Screen {
    pub fn new(mut repository: Repository, width: i32, height: i32) -> Self {
        Screen {
            repository,
            width,
            height,
        }
    }

    pub fn prepare_draw(&mut self) {
        let screen_entity = ScreenEntity::new();

        self.repository.prepare_raw::<ScreenEntity>(screen_entity);
    }

    pub fn draw(&mut self, screen_position: usize, color_id: ColorId) {
        let mut screen_entity = ScreenEntity::new();

        // TODO: solve problem with strange compiler behavior when use enum / matching!
        if color_id as u8 == 1 {
            screen_entity.psp_rgba = color::WHITE;
        }

        if color_id as u8 == 2 {
            screen_entity.psp_rgba = color::BLACK;
        }

        if color_id as u8 == 3 {
            screen_entity.psp_rgba = color::GRAY;
        }

        if color_id as u8 == 4 {
            screen_entity.psp_rgba = color::BLUE;
        }

        if color_id as u8 == 5 {
            screen_entity.psp_rgba = color::TEAL;
        }

        if color_id as u8 == 6 {
            screen_entity.psp_rgba = color::GREEN;
        }

        if color_id as u8 == 7 {
            screen_entity.psp_rgba = color::GREEN_ACID;
        }

        if color_id as u8 == 8 {
            screen_entity.psp_rgba = color::YELLOW;
        }

        if color_id as u8 == 9 {
            screen_entity.psp_rgba = color::RED;
        }

        if color_id as u8 == 10 {
            screen_entity.psp_rgba = color::MAROON;
        }

        if color_id as u8 == 11 {
            screen_entity.psp_rgba = color::PURPLE;
        }

        self.repository.update_raw::<ScreenEntity>(screen_position, screen_entity);
    }
}