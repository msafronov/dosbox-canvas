use crate::Repository;
use crate::ScreenZIndexEntity;

#[derive(Clone, Copy)]
pub struct ScreenZIndex {
    repository: Repository,
    width: i32,
    height: i32,
    z_index_count: i32,
}

impl ScreenZIndex {
    pub fn new(repository: Repository, width: i32, height: i32, z_index_count: i32) -> Self {
        ScreenZIndex {
            repository,
            width,
            height,
            z_index_count,
        }
    }
}