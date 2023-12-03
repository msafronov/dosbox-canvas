use data_access_layer::ElementEntity;

use crate::ElementDTO;

#[derive(Clone, Copy)]
pub struct DtoToEntityMapper {}

impl DtoToEntityMapper {
    pub fn new() -> Self {
        DtoToEntityMapper {}
    }

    pub fn from_dto_to_entity(&self, element_dto: ElementDTO) -> ElementEntity {
        let mut element_entity = ElementEntity::new();

        element_entity.ep_type_id = element_dto.type_id as u8;
        element_entity.ep_enabled = element_dto.enabled;
        element_entity.ep_color_id = element_dto.color_id as u8;
        element_entity.ep_x1 = element_dto.x1;
        element_entity.ep_y1 = element_dto.y1;
        element_entity.ep_z1 = element_dto.z1;
        element_entity.ep_x2 = element_dto.x2;
        element_entity.ep_y2 = element_dto.y2;
        element_entity.ep_z2 = element_dto.z2;
        element_entity.ep_border_color_id = element_dto.border_color_id as u8;
        element_entity.ep_text = element_dto.text;

        return element_entity;
    }
}