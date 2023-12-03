mod draw;
mod axis;
mod coordinate_system;
mod rect;
mod point;

use draw::Draw;
use axis::Axis;
use coordinate_system::CoordinateSystem;
use coordinate_system::PositiveCartesianCoordinateSystem;
use rect::IRect;
use point::Point;

#[derive(Clone, Copy)]
pub struct Geometry {
    width: i32,
    height: i32,
    z_index_count: i32,
}

impl Geometry {
    pub fn new(width: i32, height: i32, z_index_count: i32) -> Self {
        Geometry {
            width,
            height,
            z_index_count,
        }
    }

    pub fn create_rect(&self, x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> impl IRect {
        let coordinate_system = PositiveCartesianCoordinateSystem::new(
            CoordinateSystem::new(
                Axis::new(0, self.width),
                Axis::new(0, self.height),
                Axis::new(0, self.z_index_count)),
        );

        let draw = Draw::new(
            coordinate_system,
        );

        let point_start = Point::new(x1, y1, z1);
        let point_end = Point::new(x2, y2, z2);

        let rect = draw.rect(point_start, point_end);

        rect
    }
}