use crate::element::element::ielement::IElement;
use crate::element::element_dto::ElementDTO;
use crate::screen_pencil::ScreenPencil;

pub struct PanelElement {
    element_dto: ElementDTO,
    screen_pencil: ScreenPencil,
}

impl PanelElement {
    pub fn new(element_dto: ElementDTO, screen_pencil: ScreenPencil) -> Self {
        PanelElement {
            element_dto,
            screen_pencil,
        }
    }

    pub fn border(
        &self,
        x_current: u16,
        y_current: u16,
        border_x_min: u16,
        border_x_max: u16,
        border_y_min: u16,
        border_y_max: u16,
    ) -> bool {
        let horizontal_line =
            x_current >= border_x_min &&
            x_current <= border_x_max &&
            (
                y_current == border_y_min ||
                y_current == border_y_max
            );

        let vertical_doubled_line =
            y_current >= border_y_min &&
            y_current <= border_y_max &&
            (
                x_current == border_x_min ||
                x_current == border_x_min + 1 ||
                x_current == border_x_max ||
                x_current == border_x_max - 1
            );

        return horizontal_line || vertical_doubled_line;
        
    }
}

impl IElement for PanelElement {
    fn render(&mut self) {
        self.screen_pencil.prepare_draw();

        let border_outer_x_min = self.element_dto.x1 + 2;
        let border_outer_x_max = self.element_dto.x2 - 3;
        let border_outer_y_min = self.element_dto.y1 + 12;
        let border_outer_y_max = self.element_dto.y2 - 15;

        let border_inner_x_min = border_outer_x_min + 3;
        let border_inner_x_max = border_outer_x_max - 3;
        let border_inner_y_min = border_outer_y_min + 2;
        let border_inner_y_max = border_outer_y_max - 2;

        for y_current in self.element_dto.y1..self.element_dto.y2 {
            for x_current in self.element_dto.x1..self.element_dto.x2 {
                let mut color = self.element_dto.color_id;

                // borders
                let outer_border = self.border(
                    x_current,
                    y_current,
                    border_outer_x_min,
                    border_outer_x_max,
                    border_outer_y_min,
                    border_outer_y_max,
                );

                let inner_border = self.border(
                    x_current,
                    y_current,
                    border_inner_x_min,
                    border_inner_x_max,
                    border_inner_y_min,
                    border_inner_y_max,
                );

                if outer_border || inner_border {
                    color = self.element_dto.border_color_id;
                }

                self.screen_pencil.draw_point(
                    x_current,
                    y_current,
                    self.element_dto.z1,
                    color,
                );
            }
        }
    }
}