use bevy::{
    input::touch::{self, Touches},
    prelude::*,
    time::Time,
};

pub struct OrbitCameraControllerPlugin;

impl Plugin for OrbitCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TouchCameraConfig::default())
            .add_systems(Update, camera_orbit);
    }
}

#[derive(Resource, Clone)]
pub struct TouchCameraConfig {
    pub orbit_speed: f32,
    pub pitch_speed: f32,
    pub zoom_speed: f32,
    pub max_pitch: f32,
    pub min_pitch: f32,
    pub max_zoom: f32,
    pub min_zoom: f32,
    pub drag_sensitivity: f32,
}

impl Default for TouchCameraConfig {
    fn default() -> Self {
        Self {
            orbit_speed: 2.0,
            pitch_speed: 1.0,
            zoom_speed: 0.2,
            max_pitch: 20.0 * std::f32::consts::PI / 180.0,
            min_pitch: -20.0 * std::f32::consts::PI / 180.0,
            max_zoom: 10.0,
            min_zoom: 2.0,
            drag_sensitivity: 0.005,
        }
    }
}

fn camera_orbit(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    //touches_res: Res<Touches>,
    mut query: Query<(&Camera, &mut Transform)>,
    config: Res<TouchCameraConfig>,
) {
    for (_camera, mut transform) in query.iter_mut() { 
        // Keyboard control
        // Y-axis rotation
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D) {
            let rotation_direction = if keyboard_input.pressed(KeyCode::A) { config.orbit_speed } else { -config.orbit_speed };
            let rotation = Quat::from_rotation_y(rotation_direction * time.delta_seconds());
            let translation = transform.translation;
            transform.translation = Vec3::ZERO;
            transform.rotate(rotation);
            transform.translation = rotation.mul_vec3(translation);
        }

        // Handle X-axis rotation with clamped pitch 
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S) {
            let current_pitch = transform.forward().y.asin();
            
            let pitch_direction = if keyboard_input.pressed(KeyCode::W) { -config.pitch_speed } else { config.pitch_speed };
            let mut proposed_pitch_change = pitch_direction * time.delta_seconds();
            let proposed_pitch = current_pitch + proposed_pitch_change;

            // Clamp the proposed pitch change
            if proposed_pitch < config.min_pitch {
                proposed_pitch_change += config.min_pitch - proposed_pitch;
            } else if proposed_pitch > config.max_pitch {
                proposed_pitch_change += config.max_pitch - proposed_pitch;
            }

            let axis_of_rotation = transform.right();
            let rotation = Quat::from_axis_angle(axis_of_rotation, proposed_pitch_change);
            
            let translation_to_origin = transform.translation - Vec3::ZERO;
            let rotated_translation = rotation.mul_vec3(translation_to_origin);
            transform.translation = Vec3::ZERO + rotated_translation;
            
            transform.rotate(rotation);
        }


        if keyboard_input.pressed(KeyCode::E) || keyboard_input.pressed(KeyCode::Q) {
            let zoom_direction = if keyboard_input.pressed(KeyCode::E) { config.zoom_speed } else { -config.zoom_speed };
            let new_zoom = transform.translation.length() + zoom_direction;
            if new_zoom >= config.min_zoom && new_zoom <= config.max_zoom {
                transform.translation = transform.translation.normalize() * new_zoom;
            }
        }

        /* 
        let touches: Vec<&touch::Touch> = touches_res.iter().collect();

        // Touch control logic
        if touches.len() == 2 {
            // Implement pinch to zoom
            let distance_current = touches[0].position() - touches[1].position();
            let distance_prev = touches[0].previous_position() - touches[1].previous_position();
            let pinch_direction = distance_prev.length() - distance_current.length();
            let zoom_amount = pinch_direction * config.zoom_speed;
            let forward = transform.forward();
            transform.translation += forward * zoom_amount;

            // Clamp the zoom
            let distance = transform.translation.length();
            if distance < config.min_zoom {
                transform.translation = transform.translation.normalize() * config.min_zoom;
            } else if distance > config.max_zoom {
                transform.translation = transform.translation.normalize() * config.max_zoom;
            }
        } else if touches.len() == 1 {
            // Implement orbit
            let delta = touches[0].delta();
            let rotation_x = Quat::from_rotation_y(-delta.x * config.orbit_speed * time.delta_seconds());
            let rotation_y = Quat::from_rotation_x(-delta.y * config.pitch_speed * time.delta_seconds());

            transform.rotate(rotation_x);
            transform.rotate(rotation_y);

            // Clamp the pitch
            let current_pitch = transform.rotation.to_euler(EulerRot::XYZ).1;
            if current_pitch < config.min_pitch {
                transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, config.min_pitch, 0.0);
            } else if current_pitch > config.max_pitch {
                transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, config.max_pitch, 0.0);
            }
        }
        */
    }
}


/*
fn camera_orbit(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    const ORBIT_SPEED: f32 = 2.0;
    const PITCH_SPEED: f32 = 1.0; // Reduced pitch step for finer control
    const ZOOM_SPEED: f32 = 0.2;
    const MAX_PITCH: f32 = 20.0 * std::f32::consts::PI / 180.0; // 20 degrees in radians
    const MIN_PITCH: f32 = -20.0 * std::f32::consts::PI / 180.0; // -20 degrees in radians
    const MAX_ZOOM: f32 = 10.0; // maximum distance from the object
    const MIN_ZOOM: f32 = 2.0;  // minimum distance from the object


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

        // Zoom in/out with limits
        if keyboard_input.pressed(KeyCode::E) || keyboard_input.pressed(KeyCode::Q) {
            let zoom_direction = if keyboard_input.pressed(KeyCode::E) { 1.0 } else { -1.0 };
            let proposed_translation = transform.translation + transform.forward() * zoom_direction * ZOOM_SPEED;

            // Calculate the distance from the object
            let distance = proposed_translation.length();

            // Apply zoom only if within limits
            if distance >= MIN_ZOOM && distance <= MAX_ZOOM {
                transform.translation = proposed_translation;
            }
        }
    }
}
*/