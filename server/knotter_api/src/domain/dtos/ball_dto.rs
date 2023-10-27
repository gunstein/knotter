use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::dtos::impulse_dto::ImpulseDto;
use crate::domain::dtos::position_dto::PositionDto;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct BallDto {
    pub is_fixed: bool,
    pub is_insert: bool,
    pub uuid: Uuid,
    pub color: Option<String>, 
    pub position: Option<PositionDto>,
    pub impulse: Option<ImpulseDto>,
}