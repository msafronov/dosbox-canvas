pub trait ICoordinateSystem {
    fn x_min(&self) -> i32;
    fn x_max(&self) -> i32;
    fn y_min(&self) -> i32;
    fn y_max(&self) -> i32;
    fn z_min(&self) -> i32;
    fn z_max(&self) -> i32;
}