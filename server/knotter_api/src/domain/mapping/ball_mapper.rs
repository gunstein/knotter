use crate::domain::dtos::insert_ball_dto::{InsertBallDto, InsertPositionDto, InsertImpulseDto};
use crate::domain::models::ball_entity::{BallEntity, PositionEntity, ImpulseEntity};

pub fn dto_to_entity(dto: &InsertBallDto) -> BallEntity {
    BallEntity {
        is_fixed: dto.is_fixed,
        is_insert: dto.is_insert,
        uuid: dto.uuid,
        color: dto.color.clone(),
        position: dto.position.as_ref().map(|pos| PositionEntity {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }),
        impulse: dto.impulse.as_ref().map(|imp| ImpulseEntity {
            x: imp.x,
            y: imp.y,
            z: imp.z,
        }),
    }
}

pub fn entity_to_dto(entity: &BallEntity) -> InsertBallDto {
    InsertBallDto {
        is_fixed: entity.is_fixed,
        is_insert: entity.is_insert,
        uuid: entity.uuid,
        color: entity.color.clone(),
        position: entity.position.as_ref().map(|pos| InsertPositionDto {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }),
        impulse: entity.impulse.as_ref().map(|imp| InsertImpulseDto {
            x: imp.x,
            y: imp.y,
            z: imp.z,
        }),
    }
}
