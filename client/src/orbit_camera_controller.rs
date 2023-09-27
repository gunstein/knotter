use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct OrbitCameraControllerPlugin;

impl Plugin for OrbitCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, camera_orbit);
    }
}

fn camera_orbit(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    const ORBIT_SPEED: f32 = 2.0;
    const PITCH_SPEED: f32 = 1.0; // Reduced pitch step for finer control
    const ZOOM_SPEED: f32 = 0.5;
    const MAX_PITCH: f32 = 20.0 * std::f32::consts::PI / 180.0; // 20 degrees in radians
    const MIN_PITCH: f32 = -20.0 * std::f32::consts::PI / 180.0; // -20 degrees in radians

    for (_tag, mut transform) in query.iter_mut() {
        // Handle Y-axis rotation
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D) {
            let rotation_direction = if keyboard_input.pressed(KeyCode::A) { ORBIT_SPEED } else { -ORBIT_SPEED };
            let rotation = Quat::from_rotation_y(rotation_direction * time.delta_seconds());
            
            let translation = transform.translation - Vec3::ZERO;
            let rotated_translation = rotation.mul_vec3(translation);
            transform.translation = Vec3::ZERO + rotated_translation;
            transform.rotate(rotation);
        }

        // Handle X-axis rotation with clamped pitch
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S) {
            let current_pitch = transform.forward().y.asin();
            
            let pitch_direction = if keyboard_input.pressed(KeyCode::W) { -PITCH_SPEED } else { PITCH_SPEED };
            let mut proposed_pitch_change = pitch_direction * time.delta_seconds();
            let proposed_pitch = current_pitch + proposed_pitch_change;

            // Clamp the proposed pitch change
            if proposed_pitch < MIN_PITCH {
                proposed_pitch_change += MIN_PITCH - proposed_pitch;
            } else if proposed_pitch > MAX_PITCH {
                proposed_pitch_change += MAX_PITCH - proposed_pitch;
            }

            let axis_of_rotation = transform.right();
            let rotation = Quat::from_axis_angle(axis_of_rotation, proposed_pitch_change);
            
            let translation_to_origin = transform.translation - Vec3::ZERO;
            let rotated_translation = rotation.mul_vec3(translation_to_origin);
            transform.translation = Vec3::ZERO + rotated_translation;
            
            transform.rotate(rotation);
        }

        // Zoom in/out
        if keyboard_input.pressed(KeyCode::E) {
            let forward = transform.forward();
            transform.translation += forward * ZOOM_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Q) {
            let forward = transform.forward();
            transform.translation -= forward * ZOOM_SPEED;
        }
    }
}
