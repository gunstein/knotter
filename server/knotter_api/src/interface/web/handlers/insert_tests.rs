#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::domain::dtos::insert_ball_dto::InsertBallDto;
    use crate::domain::dtos::impulse_dto::ImpulseDto;
    use crate::domain::dtos::position_dto::PositionDto;
    use crate::domain::mapping::ball_mapper::{dto_to_entity, entity_to_dto};
    use crate::domain::models::ball_entity::BallEntity;

    #[test]
    fn test_insert_mapping_and_serialization() {
        let uuid = Uuid::new_v4(); // Generates a new random UUID

        let insert_ball_dto_in = InsertBallDto {
            is_insert: true,
            is_fixed: true,
            uuid: uuid,
            color: Some("red".to_string()),
            position: Some(PositionDto { x: 1.0, y: 2.0, z: 3.0 }),
            impulse: Some(ImpulseDto { x: 1.0, y: 2.0, z: 3.0 }),
        };
        
        let insert_ball_entity_in = dto_to_entity(&insert_ball_dto_in);
        
        let serialized = serde_json::to_string(&insert_ball_entity_in).expect("Failed to serialize");

        assert_eq!(serialized, format!(r#"{{"is_fixed":true,"is_insert":true,"uuid":"{}","color":"red","position":{{"x":1.0,"y":2.0,"z":3.0}},"impulse":{{"x":1.0,"y":2.0,"z":3.0}}}}}}"#, uuid.to_string()));

        let insert_ball_entity_out: BallEntity = serde_json::from_str(&serialized).expect("Failed to deserialize");

        let insert_ball_dto_out = entity_to_dto(&insert_ball_entity_out);
        
        assert_eq!(insert_ball_dto_in, insert_ball_dto_out);
    }    
}