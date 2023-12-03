use crate::geometry;
use crate::Repository;
use crate::ElementEntity;
use crate::screen_pencil::ScreenPencil;

mod element;
mod element_dto;
mod element_type_id;
mod mapper;
mod font;

use geometry::Geometry;

pub use element::*;
pub use element_dto::ElementDTO;
pub use element_type_id::ElementTypeId;
pub use mapper::DtoToEntityMapper;
pub use font::PERFECT_DOS_VGA_437_8x16_FONT;

// TODO: get rid of
enum Elem {
    PanelElement(PanelElement),
    TextElement(TextElement),
}

#[derive(Clone, Copy)]
pub struct Element {
    geometry: Geometry,
    repository: Repository,
    mapper: DtoToEntityMapper,
    screen_pencil: ScreenPencil,
    font_width: u8,
    font_height: u8,
}

impl Element {
    pub fn new(
        geometry: Geometry,
        repository: Repository,
        mapper: DtoToEntityMapper,
        screen_pencil: ScreenPencil,
        font_width: u8,
        font_height: u8,
    ) -> Self {
        Element {
            geometry,
            repository,
            mapper,
            screen_pencil,
            font_width,
            font_height,
        }
    }

    pub fn create(&self, element_dto: ElementDTO) {
        // TODO: element coordinates correction via geometry rules
        let rect = self.geometry.create_rect(
            element_dto.x1 as i32,
            element_dto.y1 as i32,
            element_dto.z1 as i32,
            element_dto.x2 as i32,
            element_dto.y2 as i32,
            element_dto.z2 as i32,
        );

        let element = match element_dto.type_id {
            ElementTypeId::Panel => Elem::PanelElement(
                PanelElement::new(
                    element_dto,
                    self.screen_pencil,
                )
            ),
            ElementTypeId::Text => Elem::TextElement(
                TextElement::new(
                    element_dto,
                    self.screen_pencil,
                    self.font_width,
                    self.font_height,
                )
            ),
            _ => Elem::PanelElement(
                PanelElement::new(
                    element_dto,
                    self.screen_pencil,
                )
            ),
        };

        let element_entity = self.mapper.from_dto_to_entity(element_dto);

        self.repository.insert::<ElementEntity>(element_entity);

        match element {
            Elem::PanelElement(mut elem) => elem.render(),
            Elem::TextElement(mut elem) => elem.render(),
        };
    }
}