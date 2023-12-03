#[derive(Clone, Copy)]
pub struct GeometryToScreenMapper {
    width: i32,
    height: i32,
    screen_pos_size: usize,
}

impl GeometryToScreenMapper {
    pub fn new(width: i32, height: i32, screen_pos_size: usize) -> Self {
        GeometryToScreenMapper {
            width,
            height,
            screen_pos_size,
        }
    }

    pub fn from_geometry_to_screen_pos(&self, x: u16, y: u16) -> usize {
        ((self.width * y as i32) + x as i32) as usize * self.screen_pos_size
    }
}