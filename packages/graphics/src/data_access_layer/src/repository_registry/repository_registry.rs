use crate::FontEntity;
use crate::IEntity;
use crate::MemoryLayer;
use crate::Repository;
use crate::ScreenEntity;
use crate::ScreenZIndexEntity;
use crate::ElementEntity;

#[derive(Clone, Copy)]
pub struct RepositoryRegistry {
    memory_layer: MemoryLayer,
}

impl RepositoryRegistry {
    pub fn new(memory_layer: MemoryLayer) -> Self {
        RepositoryRegistry {
            memory_layer,
        }
    }

    pub fn init(&mut self) {
        self.create_memory_block_for_font();
        self.create_memory_block_for_screen();
        self.create_memory_block_for_screen_z_index();
        self.create_memory_block_for_element();
    }

    pub fn get_repository(&self) -> Repository {
        let repository = Repository::new(self.memory_layer);

        repository
    }

    fn create_memory_block_for_font(&mut self) {
        let font_entity = FontEntity::new();

        self.memory_layer.memory_block_create(
            font_entity._id() as u8,
            font_entity._len(),
            0,
        );
    }

    fn create_memory_block_for_screen(&mut self) {
        let screen_entity = ScreenEntity::new();

        self.memory_layer.memory_block_create(
            screen_entity._id() as u8,
            screen_entity._len(),
            640 * 480,
        );
    }

    fn create_memory_block_for_screen_z_index(&mut self) {
        let screen_z_index_entity = ScreenZIndexEntity::new();

        self.memory_layer.memory_block_create(
            screen_z_index_entity._id() as u8,
            screen_z_index_entity._len(),
            640 * 480,
        );
    }

    fn create_memory_block_for_element(&mut self) {
        let element_entity = ElementEntity::new();

        self.memory_layer.memory_block_create(
            element_entity._id() as u8,
            element_entity._len(),
            0,
        );
    }
}