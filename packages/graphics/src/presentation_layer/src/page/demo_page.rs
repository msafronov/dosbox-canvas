use super::IPage;
use crate::ElementDTO;
use crate::ElementTypeId;
use crate::ColorId;
use crate::ApplicationLayer;

pub struct DemoPage {
    application_layer: ApplicationLayer,
}

impl DemoPage {
    pub fn new(application_layer: ApplicationLayer) -> Self {
        DemoPage {
            application_layer,
        }
    }

    fn create_left_panel(&self) {
        let mut panel_element_dto = ElementDTO::new();

        panel_element_dto.type_id = ElementTypeId::Panel;
        panel_element_dto.color_id = ColorId::Blue;
        panel_element_dto.border_color_id = ColorId::Teal;
        panel_element_dto.x1 = 0;
        panel_element_dto.y1 = 0;
        panel_element_dto.x2 = 320;
        panel_element_dto.y2 = 480;

        self.application_layer.create_element(panel_element_dto);
    }

    fn create_left_panel_title(&self) {
        // bg panel
        let mut panel_element_dto = ElementDTO::new();

        panel_element_dto.type_id = ElementTypeId::Panel;
        panel_element_dto.color_id = ColorId::Green;
        panel_element_dto.border_color_id = ColorId::Green;
        panel_element_dto.x1 = 95;
        panel_element_dto.y1 = 3;
        panel_element_dto.x2 = 225;
        panel_element_dto.y2 = 23;

        self.application_layer.create_element(panel_element_dto);

        // title text
        let mut title_text_element_dto = ElementDTO::new();

        title_text_element_dto.type_id = ElementTypeId::Text;
        title_text_element_dto.color_id = ColorId::Blue;
        title_text_element_dto.x1 = 106;
        title_text_element_dto.y1 = 6;
        title_text_element_dto.x2 = 210;
        title_text_element_dto.y2 = 26;
        title_text_element_dto.text = self.application_layer.create_text(
            "DOSBOX CANVAS",
        );

        self.application_layer.create_element(title_text_element_dto);
    }

    fn create_right_panel(&self) {
        let mut panel_element_dto = ElementDTO::new();

        panel_element_dto.type_id = ElementTypeId::Panel;
        panel_element_dto.color_id = ColorId::Blue;
        panel_element_dto.border_color_id = ColorId::Teal;
        panel_element_dto.x1 = 320;
        panel_element_dto.y1 = 0;
        panel_element_dto.x2 = 640;
        panel_element_dto.y2 = 480;

        self.application_layer.create_element(panel_element_dto);
    }

    fn create_right_panel_title(&self) {
        // bg panel
        let mut panel_element_dto = ElementDTO::new();

        panel_element_dto.type_id = ElementTypeId::Panel;
        panel_element_dto.color_id = ColorId::Blue;
        panel_element_dto.border_color_id = ColorId::Blue;
        panel_element_dto.x1 = 450;
        panel_element_dto.y1 = 3;
        panel_element_dto.x2 = 510;
        panel_element_dto.y2 = 23;

        self.application_layer.create_element(panel_element_dto);

        // title text
        let mut title_text_element_dto = ElementDTO::new();

        title_text_element_dto.type_id = ElementTypeId::Text;
        title_text_element_dto.color_id = ColorId::Teal;
        title_text_element_dto.x1 = 465;
        title_text_element_dto.y1 = 6;
        title_text_element_dto.x2 = 510;
        title_text_element_dto.y2 = 26;
        title_text_element_dto.text = self.application_layer.create_text(
            "Info",
        );

        self.application_layer.create_element(title_text_element_dto);

        // info text
        let mut info_text_element_dto = ElementDTO::new();

        info_text_element_dto.type_id = ElementTypeId::Text;
        info_text_element_dto.color_id = ColorId::Yellow;
        info_text_element_dto.x1 = 405;
        info_text_element_dto.y1 = 60;
        info_text_element_dto.x2 = 510;
        info_text_element_dto.y2 = 80;
        info_text_element_dto.text = self.application_layer.create_text(
            "Work in Progress :E",
        );

        self.application_layer.create_element(info_text_element_dto);
    }
}

impl IPage for DemoPage {
    fn init(&self) {
        self.create_left_panel();
        self.create_left_panel_title();
        self.create_right_panel();
        self.create_right_panel_title();
    }

    fn render(&self) {
        //
    }
}