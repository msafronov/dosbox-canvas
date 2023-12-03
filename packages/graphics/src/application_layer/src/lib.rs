#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod config;
mod color;
mod color_id;
mod geometry;
mod screen;
mod screen_z_index;
mod screen_pencil;
mod element;
mod app_renderer;

use data_access_layer::DataAccessLayer;
use data_access_layer::IEntity;
use data_access_layer::Repository;
use data_access_layer::ElementEntity;
use data_access_layer::ScreenEntity;
use data_access_layer::ScreenZIndexEntity;
use geometry::Geometry;
use screen::Screen;
use screen_z_index::ScreenZIndex;
use screen_pencil::ScreenPencil;
use screen_pencil::GeometryToScreenMapper;
use element::Element;
use element::DtoToEntityMapper;
use app_renderer::AppRenderer;

pub use color_id::ColorId;
pub use element::*;

#[derive(Clone, Copy)]
pub struct ApplicationLayer {
    data_access_layer: DataAccessLayer,
    element: Element,
    app_renderer: AppRenderer,
}

impl ApplicationLayer {
    pub fn new(data_access_layer: DataAccessLayer) -> Self {
        let geometry = Geometry::new(
            config::SCREEN_WIDTH,
            config::SCREEN_HEIGHT,
            config::SCREEN_Z_INDEX_COUNT,
        );

        let screen = Screen::new(
            data_access_layer.get_repository(),
            config::SCREEN_WIDTH,
            config::SCREEN_HEIGHT,
        );

        let screen_z_index = ScreenZIndex::new(
            data_access_layer.get_repository(),
            config::SCREEN_WIDTH,
            config::SCREEN_HEIGHT,
            config::SCREEN_Z_INDEX_COUNT,
        );

        let screen_entity = ScreenEntity::new();

        let dto_to_entity_mapper = DtoToEntityMapper::new();

        let geometry_to_screen_mapper = GeometryToScreenMapper::new(
            config::SCREEN_WIDTH,
            config::SCREEN_HEIGHT,
            screen_entity._len(),
        );

        let screen_pencil = ScreenPencil::new(
            screen,
            screen_z_index,
            geometry_to_screen_mapper,
        );

        let element = Element::new(
            geometry,
            data_access_layer.get_repository(),
            dto_to_entity_mapper,
            screen_pencil,
            config::FONT_WIDTH,
            config::FONT_HEIGHT,
        );

        let app_renderer = AppRenderer::new();

        ApplicationLayer {
            data_access_layer,
            element,
            app_renderer,
        }
    }

    pub fn init(&mut self) {
        self.data_access_layer.init();
    }

    pub fn render(&self) {
        self.app_renderer.render();
    }

    pub fn screen_width(&self) -> i32 {
        config::SCREEN_WIDTH
    }

    pub fn screen_height(&self) -> i32 {
        config::SCREEN_HEIGHT
    }

    pub fn create_element(&self, element_dto: ElementDTO) {
        self.element.create(element_dto);
    }

    pub fn create_text(&self, str: &str) -> [u8; 40] {
        let mut result: [u8; 40] = [0; 40];

        let mut counter: usize = 0;

        for char in str.chars() {
            result[counter] = char as u8;

            counter = counter + 1;
        }

        return result;
    }
}