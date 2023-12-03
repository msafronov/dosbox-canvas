use crate::ApplicationLayer;

use crate::page::IPage;
use crate::page::DemoPage;

pub struct Pages {
    demo_page: DemoPage,
}

impl Pages {
    pub fn new(application_layer: ApplicationLayer) -> Self {
        let demo_page = DemoPage::new(
            application_layer,
        );

        Pages {
            demo_page,
        }
    }

    pub fn init(&self) {
        self.demo_page.init();
    }

    pub fn render(&self) {
        self.demo_page.render();
    }
}