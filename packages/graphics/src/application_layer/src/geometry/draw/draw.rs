use super::IDraw;

use crate::geometry::coordinate_system::ICoordinateSystem;
use crate::geometry::rect::IRect;
use crate::geometry::rect::Rect;
use crate::geometry::rect::FlatRect;
use crate::geometry::point::Point;

pub struct Draw<T> {
    coordinate_system: T,
}

impl<T: ICoordinateSystem> Draw<T> {
    pub fn new(coordinate_system: T) -> Self {
        Draw {
            coordinate_system,
        }
    }

    pub fn rect(&self, point_start: Point, point_end: Point) -> impl IRect {
        let rect = FlatRect::new(
            Rect::new(
                point_start,
                point_end,
            ),
        );

        if rect.x1() < self.coordinate_system.x_min() {
            // error?
        }

        if rect.y1() < self.coordinate_system.y_min() {
            // error?
        }

        if rect.z1() < self.coordinate_system.z_min() {
            // error?
        }

        if rect.x2() > self.coordinate_system.x_max() {
            // error?
        }

        if rect.y2() > self.coordinate_system.y_max() {
            // error?
        }

        if rect.z2() > self.coordinate_system.z_max() {
            // error?
        }

        rect
    }
}

impl<T: ICoordinateSystem> IDraw for Draw<T> {}