use super::IAxis;

pub struct Axis {
    min: i32,
    max: i32,
}

impl Axis {
    pub fn new(min: i32, max: i32) -> Self {
        Axis {
            min,
            max,
        }
    }
}

impl IAxis for Axis {
    fn min(&self) -> i32 {
        self.min
    }

    fn max(&self) -> i32 {
        self.max
    }
}