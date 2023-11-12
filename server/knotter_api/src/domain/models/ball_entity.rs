
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use nalgebra::Vector3;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct BallEntity {
    pub is_fixed: bool,
    pub is_insert: bool,
    pub uuid: Uuid,
    pub color: Option<String>, 
    pub position: Option<PositionEntity>,
    pub impulse: Option<ImpulseEntity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PositionEntity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PositionEntity {
    pub fn distance_squared(&self, other: &PositionEntity) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx*dx + dy*dy + dz*dz
    }

    // Conversion to nalgebra::Vector3<f64>
    pub fn to_vector3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ImpulseEntity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ImpulseEntity {
    // Conversion to nalgebra::Vector3<f64>
    pub fn to_vector3(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl BallEntity {
    pub fn new(uuid: Uuid, is_insert: bool) -> Self {
        BallEntity {
            is_fixed: false,
            is_insert,
            uuid: uuid,
            color: None,
            position: None,
            impulse: None,
        }
    }
}