use errors::Error;

use crate::geometry::point::Point;
use crate::geometry::rect::IRect;

pub struct Rect {
    point_start: Point,
    point_end: Point,
}

impl Rect {
    pub fn new(
        point_start: Point,
        point_end: Point,
    ) -> Self {
        if point_start.x() > point_end.x() {
            errors::panic(Error::RectInvalidNegativeWidth);
        }

        if point_start.y() > point_end.y() {
            errors::panic(Error::RectInvalidNegativeHeight);
        }

        Rect {
            point_start,
            point_end,
        }
    }
}

impl IRect for Rect {
    fn x1(&self) -> i32 {
        self.point_start.x()
    }

    fn y1(&self) -> i32 {
        self.point_start.y()
    }

    fn z1(&self) -> i32 {
        self.point_start.z()
    }

    fn x2(&self) -> i32 {
        self.point_end.x()
    }

    fn y2(&self) -> i32 {
        self.point_end.y()
    }

    fn z2(&self) -> i32 {
        self.point_end.z()
    }
}