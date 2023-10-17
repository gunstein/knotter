use crate::serialization::*;

pub struct Sphere {
    pub center: Position,
    pub radius: f64,
}

impl Sphere {
    pub fn contains(&self, point: &Position) -> bool {
        point.distance_squared(&self.center) == self.radius*self.radius
    }
}

pub fn is_valid_distance_from_others(point: &Position, others: &Vec<&Position>, min_distance: f64) -> bool {
    let min_distance_squared = min_distance * min_distance;
    for other_point in others {
        if point.distance_squared(other_point) < min_distance_squared {
            return false;
        }
    }
    true
}

/* 
fn main() {
    let sphere = Sphere {
        center: Point3D { x: 0.0, y: 0.0, z: 0.0 },
        radius: 1.0,
    };
    
    let other_points = vec![
        Point3D { x: 1.0, y: 0.0, z: 0.0 },
        Point3D { x: -1.0, y: 0.2, z: 0.1 },
        // Add more points here
    ];

    let given_point = Point3D { x: 0.5, y: 0.5, z: 0.5 };
    
    if sphere.contains(&given_point) && is_valid_distance_from_others(&given_point, &other_points, 0.5 /* min_distance */) {
        println!("The given point is valid.");
    } else {
        println!("The given point does not meet the criteria.");
    }
}
*/