use crate::domain::dto::ball_dto::{BallDataDto, DtoPosition, DtoImpulse};
use crate::domain::models::ball_entity::{BallEntity, EntityPosition, EntityImpulse};

pub fn dto_to_entity(dto: &BallDto) -> BallEntity {
    BallEntity {
        is_fixed: dto.is_fixed,
        is_insert: dto.is_insert,
        uuid: dto.uuid,
        color: dto.color.clone(),
        position: dto.position.as_ref().map(|pos| EntityPosition {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }),
        impulse: dto.impulse.as_ref().map(|imp| EntityImpulse {
            x: imp.x,
            y: imp.y,
            z: imp.z,
        }),
    }
}

pub fn entity_to_dto(entity: &BallEntity) -> BallDto {
    BallDto {
        is_fixed: entity.is_fixed,
        is_insert: entity.is_insert,
        uuid: entity.uuid,
        color: entity.color.clone(),
        position: entity.position.as_ref().map(|pos| DtoPosition {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }),
        impulse: entity.impulse.as_ref().map(|imp| DtoImpulse {
            x: imp.x,
            y: imp.y,
            z: imp.z,
        }),
    }
}
