#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

mod page;
mod pages;

use application_layer::ApplicationLayer;
use application_layer::ElementDTO;
use application_layer::ElementTypeId;
use application_layer::ColorId;
use pages::Pages;

pub struct PresentationLayer {
    application_layer: ApplicationLayer,
    pages: Pages,
}

impl PresentationLayer {
    pub fn new(application_layer: ApplicationLayer) -> Self {
        let pages = Pages::new(
            application_layer,
        );

        PresentationLayer {
            application_layer,
            pages,
        }
    }

    pub fn init(&mut self) {
        self.application_layer.init();
        self.pages.init();
    }

    pub fn run(&self) {
        self.application_layer.render();
    }

    pub fn screen_width(&self) -> i32 {
        self.application_layer.screen_width()
    }

    pub fn screen_height(&self) -> i32 {
        self.application_layer.screen_height()
    }
}