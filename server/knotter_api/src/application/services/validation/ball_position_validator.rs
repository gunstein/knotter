use crate::domain::models::ball_entity::PositionEntity;

const GLOBE_RADIUS: f32 = 1.0;
const BALL_RADIUS: f32 = 0.05;
const TOLERANCE: f32 = 0.001; // small limit above the sphere
const GLOBE_POSITION: PositionEntity = PositionEntity { x: 0.0, y: 0.0, z: 0.0 };

pub struct Globe;

impl Globe {
    pub fn contains(ball: &PositionEntity) -> bool {
        let distance_from_center = ball.distance_squared(&GLOBE_POSITION).sqrt();
        let lower_bound = GLOBE_RADIUS;
        let upper_bound = GLOBE_RADIUS + BALL_RADIUS + TOLERANCE;

        lower_bound <= distance_from_center && distance_from_center <= upper_bound
    }
}

const MIN_DISTANCE: f32 = 0.1;
const MIN_DISTANCE_SQUARED: f32 = MIN_DISTANCE * MIN_DISTANCE;

pub fn is_valid_distance_from_others(point: &PositionEntity, others: &Vec<&PositionEntity>) -> bool {
    for other_point in others {
        if point.distance_squared(other_point) < MIN_DISTANCE_SQUARED {
            return false;
        }
    }
    true
}
