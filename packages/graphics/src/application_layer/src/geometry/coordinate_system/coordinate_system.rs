use crate::geometry::axis::IAxis;
use crate::geometry::axis::Axis;
use crate::geometry::coordinate_system::ICoordinateSystem;

pub struct CoordinateSystem {
    axis_x: Axis,
    axis_y: Axis,
    axis_z: Axis,
}

impl CoordinateSystem {
    pub fn new(axis_x: Axis, axis_y: Axis, axis_z: Axis) -> Self {
        CoordinateSystem {
            axis_x,
            axis_y,
            axis_z,
        }
    }
}

impl ICoordinateSystem for CoordinateSystem {
    fn x_min(&self) -> i32 {
        self.axis_x.min()
    }

    fn x_max(&self) -> i32 {
        self.axis_x.max()
    }

    fn y_min(&self) -> i32 {
        self.axis_y.min()
    }

    fn y_max(&self) -> i32 {
        self.axis_y.max()
    }

    fn z_min(&self) -> i32 {
        self.axis_z.min()
    }

    fn z_max(&self) -> i32 {
        self.axis_z.max()
    }
}