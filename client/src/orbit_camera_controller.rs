use bevy::{
    input::touch::{self, Touches},
    prelude::*,
    time::Time,
};
use crate::AppState;
use crate::globe;
use bevy_rapier3d::prelude::*;
pub struct OrbitCameraControllerPlugin;

impl Plugin for OrbitCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TouchCameraConfig::default())
            //.add_systems(Update, camera_orbit)
            //.add_systems(Update, camera_orbit.run_if(is_not_in_desired_state))
            .add_systems(Update, camera_orbit_orbiting.run_if(in_state(AppState::Orbiting)))
            .add_systems(Update, camera_orbit_zooming.run_if(in_state(AppState::Zooming)))
            .add_systems(Update, camera_orbit_key.run_if(in_state(AppState::EditUpsert)))
            .add_systems(Update, state_handler_orbit_and_zoom.run_if(in_state(AppState::EditUpsert)
                .or_else(in_state(AppState::Zooming)).or_else(in_state(AppState::Orbiting))))
            ;
            //.add_systems(Update, print_state);
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

fn is_not_in_desired_state(state: Res<State<AppState>>) -> bool {
    match state.get() {
        AppState::EditUpsertSetSpeed => false,
        _ => true,
    }
}

fn print_state(state: Res<State<AppState>>) {
    bevy::log::info!("Current AppState: {:?}", state.get());
}

fn state_handler_orbit_and_zoom(
    touches_res: Res<Touches>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
) {
    let touches: Vec<&touch::Touch> = touches_res.iter().collect();
    if touches.len() == 2 {
        if *current_state != AppState::Zooming {
            next_state.set(AppState::Zooming);
        }
    }
    else if touches.len() == 1 {
        bevy::log::info!("touch 1");
        if *current_state != AppState::Orbiting {
            bevy::log::info!("NOT orbiting");
            if let Some(cursor_position) = touches.iter().next().map(|touch| touch.position()){
                for (camera, camera_transform) in &cameras {
                    if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                        //Only hit globe, globe is only member of CollisionGroup GROUP_1
                        let filter = QueryFilter {
                            groups: Some(
                                CollisionGroups {
                                    memberships: Group::GROUP_2,
                                    filters: Group::GROUP_1,
                                }
                            ),
                            ..default()
                        };
                        //println!("gvtest3");
                        // Then cast the ray. 
                        if let Some((_, _)) = rapier_context.cast_ray(
                            ray.origin,
                            ray.direction,
                            f32::MAX,
                            true,
                            filter,
                        ){
                            //Do nothing. let state be what it was
                            bevy::log::info!("Hit globe");
                        }
                        else{
                            bevy::log::info!("Next State orbiting");
                            //no hit
                            next_state.set(AppState::Orbiting);
                        }
                    }
                }
            }
        }
    }
    else{
        if *current_state != AppState::EditUpsert {
            bevy::log::info!("Next State EditUpsert");
            next_state.set(AppState::EditUpsert); //or EditDelete if relevant
        }
    }
}


fn camera_orbit_key(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
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
    }
}

fn camera_orbit_zooming(
    touches_res: Res<Touches>,
    mut query: Query<(&Camera, &mut Transform)>,
    config: Res<TouchCameraConfig>,
) {
    let touches: Vec<&touch::Touch> = touches_res.iter().collect();
    // Touch control logic
    if touches.len() == 2 {
        for (_camera, mut transform) in query.iter_mut() { 
            // Implement pinch to zoom
            let distance_current = touches[0].position() - touches[1].position();
            let distance_prev = touches[0].previous_position() - touches[1].previous_position();
            //let pinch_direction = distance_prev.length() - distance_current.length();
            let pinch_direction =  distance_current.length() - distance_prev.length();
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
            //next_state.set(AppState::Zooming);
        }
    }
}



fn camera_orbit_orbiting(
    time: Res<Time>,
    touches_res: Res<Touches>,
    mut query: Query<(&Camera, &mut Transform)>,
    config: Res<TouchCameraConfig>,
) {
    bevy::log::info!("ORBITING");
    let touches: Vec<&touch::Touch> = touches_res.iter().collect();

    if touches.len() == 1 {
        for (_camera, mut transform) in query.iter_mut() { 
            bevy::log::info!("ORBITING");
            let delta = touches[0].delta();

            // Calculate the proposed pitch change (rotation around the x-axis)
            let axis_of_rotation_x = transform.right();
            let mut proposed_pitch_change = -delta.y * config.pitch_speed * time.delta_seconds();
            let current_pitch = transform.forward().y.asin();
            let proposed_pitch = current_pitch + proposed_pitch_change;
        
            // Clamp the pitch change
            if proposed_pitch < config.min_pitch {
                proposed_pitch_change = config.min_pitch - current_pitch;
            } else if proposed_pitch > config.max_pitch {
                proposed_pitch_change = config.max_pitch - current_pitch;
            }
        
            // Apply the clamped pitch rotation
            let rotation_x = Quat::from_axis_angle(axis_of_rotation_x, proposed_pitch_change);
            transform.rotate(rotation_x);
        
            // Calculate and apply the rotation around the y-axis (orbit)
            let rotation_y = Quat::from_rotation_y(-delta.x * config.orbit_speed * time.delta_seconds());
            transform.rotate(rotation_y);
        
            // Adjust the camera's position and orientation
            let translation = transform.translation;
            transform.translation = Vec3::ZERO;
            transform.rotate(rotation_y * rotation_x);
            transform.translation = rotation_y * rotation_x * translation;
        
            // Ensure the camera is always oriented towards the center of the globe
            transform.look_at(Vec3::ZERO, Vec3::Y);       
        
            //bevy::log::info!("Next State orbiting");
            //next_state.set(AppState::Orbiting);
        }
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