use errors::Error;

use crate::geometry::rect::IRect;

pub struct FlatRect<T: IRect> {
    rect: T,
}

impl<T: IRect> FlatRect<T> {
    pub fn new(rect: T) -> Self {
        if rect.z1() != rect.z2() {
            errors::panic(Error::FlatRectInvalid);
        }

        FlatRect {
            rect,
        }
    }
}

impl<T: IRect> IRect for FlatRect<T> {
    fn x1(&self) -> i32 {
        self.rect.x1()
    }

    fn y1(&self) -> i32 {
        self.rect.y1()
    }

    fn z1(&self) -> i32 {
        self.rect.z1()
    }

    fn x2(&self) -> i32 {
        self.rect.x2()
    }

    fn y2(&self) -> i32 {
        self.rect.y2()
    }

    fn z2(&self) -> i32 {
        self.rect.z2()
    }
}