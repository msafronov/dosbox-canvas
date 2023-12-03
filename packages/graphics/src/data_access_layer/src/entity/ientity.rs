use crate::entity_id::EntityId;

pub trait IEntity {
    fn _id(&self) -> EntityId;
    fn _len(&self) -> usize;
}