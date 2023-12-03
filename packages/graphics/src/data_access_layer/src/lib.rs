#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod repository;
mod repository_registry;
mod entity_id;
mod entity;

use memory_layer::MemoryLayer;
use repository_registry::RepositoryRegistry;

pub use repository::Repository;
pub use entity::*;

#[derive(Clone, Copy)]
pub struct DataAccessLayer {
    memory_layer: MemoryLayer,
    repository_registry: RepositoryRegistry,
}

impl DataAccessLayer {
    pub fn new(memory_layer: MemoryLayer) -> Self {
        let repository_registry = RepositoryRegistry::new(
            memory_layer,
        );

        DataAccessLayer {
            memory_layer,
            repository_registry,
        }
    }

    pub fn init(&mut self) {
        self.memory_layer.init();
        self.repository_registry.init();
    }

    pub fn get_repository(&self) -> Repository {
        self.repository_registry.get_repository()
    }
}