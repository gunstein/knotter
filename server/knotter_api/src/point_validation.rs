use crate::serialization::*;

const GLOBE_RADIUS: f64 = 1.0;
const BALL_RADIUS: f64 = 0.05;
const TOLERANCE: f64 = 0.001; // small limit above the sphere
const GLOBE_POSITION: Position = Position { x: 0.0, y: 0.0, z: 0.0 };

pub struct Globe;

impl Globe {
    pub fn contains(ball: &Ball) -> bool {
        let distance_from_center = ball.center.distance_squared(&GLOBE_POSITION).sqrt();
        let lower_bound = GLOBE_RADIUS;
        let upper_bound = GLOBE_RADIUS + BALL_RADIUS + TOLERANCE;

        lower_bound <= distance_from_center && distance_from_center <= upper_bound
    }
}

pub struct Ball {
    pub center: Position,  // Ball owns its Position
    pub radius: f64,
}

impl Ball {
    pub fn new(center: &Position) -> Self {  // Accept a reference to a Position
        Ball {
            center: center.clone(),  // Clone the referenced Position
            radius: BALL_RADIUS,
        }
    }
}

const MIN_DISTANCE: f64 = 1.05;
const MIN_DISTANCE_SQUARED: f64 = MIN_DISTANCE * MIN_DISTANCE;

pub fn is_valid_distance_from_others(point: &Position, others: &Vec<&Position>) -> bool {
    for other_point in others {
        if point.distance_squared(other_point) < MIN_DISTANCE_SQUARED {
            return false;
        }
    }
    true
}
