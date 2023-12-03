use errors::Error;
use crate::geometry::coordinate_system::ICoordinateSystem;

pub struct PositiveCartesianCoordinateSystem<T: ICoordinateSystem> {
    coordinate_system: T,
}

impl<T: ICoordinateSystem> PositiveCartesianCoordinateSystem<T> {
    pub fn new(coordinate_system: T) -> Self {
        // origin should be at (0, 0, 0)
        if coordinate_system.x_min() != 0 {
            errors::panic(Error::CoordinateSystemInvalidOrigin);
        }

        if coordinate_system.y_min() != 0 {
            errors::panic(Error::CoordinateSystemInvalidOrigin);
        }

        if coordinate_system.z_min() != 0 {
            errors::panic(Error::CoordinateSystemInvalidOrigin);
        }

        // abscissa should be positive
        if coordinate_system.x_max() <= 0 {
            errors::panic(Error::CoordinateSystemInvalidAbscissa);
        }

        // ordinate should be positive
        if coordinate_system.y_max() <= 0 {
            errors::panic(Error::CoordinateSystemInvalidOrdinate);
        }

        // applicate should be positive
        if coordinate_system.z_max() <= 0 {
            errors::panic(Error::CoordinateSystemInvalidApplicate);
        }

        PositiveCartesianCoordinateSystem {
            coordinate_system,
        }
    }
}

impl<T: ICoordinateSystem> ICoordinateSystem for PositiveCartesianCoordinateSystem<T> {
    fn x_min(&self) -> i32 {
        self.coordinate_system.x_min()
    }

    fn x_max(&self) -> i32 {
        self.coordinate_system.x_max()
    }

    fn y_min(&self) -> i32 {
        self.coordinate_system.y_min()
    }

    fn y_max(&self) -> i32 {
        self.coordinate_system.y_max()
    }

    fn z_min(&self) -> i32 {
        self.coordinate_system.z_min()
    }

    fn z_max(&self) -> i32 {
        self.coordinate_system.z_max()
    }
}