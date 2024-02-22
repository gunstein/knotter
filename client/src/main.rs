use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

use globe::GlobePlugin;
use ball::BallPlugin;
use orbit_camera_controller::OrbitCameraControllerPlugin;
use query_server::QueryServerPlugin;
use ui::GridMenuPlugin;
use std::path::Path;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use web_sys::window;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

use std::f32::consts::PI;

mod globe;
mod ball;
mod query_server;
mod orbit_camera_controller;
mod ui;


#[cfg(target_arch = "wasm32")]
fn get_query_param(param_name: &str) -> Option<String> {
    if let Some(window) = window() {
        if let Ok(query_string) = window.location().search() {
            query_string
                .trim_start_matches('?') // Remove the leading '?' from the query string
                .split('&') // Split the query string into key-value pairs
                .filter_map(|pair| {
                    let mut parts = pair.split('=');
                    if let Some(key) = parts.next() {
                        if let Some(val) = parts.next() {
                            if key == param_name {
                                return Some(val.to_string());
                            }
                        }
                    }
                    None
                })
                .next() // Take the first occurrence
        } else {
            None
        }
    } else {
        None
    }
}


#[cfg(not(target_arch = "wasm32"))]
fn get_query_param(param_name: &str) -> Option<String> {
    Some("guni12guni".to_string())
}

#[cfg(target_arch = "wasm32")]
fn get_api_url() -> String {
    if let Some(window) = window() {
        if let Some(my_constant) = window.get("API_URL") {
            if let Some(string) = my_constant.as_string() {
                console::log_1(&format!("Constant API_URL: {}", string).into());
                string
            } else {
                println!("Constant API_URL is not a string");
                "API_URL not a string".to_string()
            }
        } else {
            //println!("Constant API_URL not found");
            console::log_1(&format!("Constant API_URL not found.").into());
            "Constant API_URL not found".to_string()
        }
    } else {
        println!("No global window object available");
        "No global window object available".to_string()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_api_url() -> String {
    "http://192.168.86.166:8080".to_string()
}

#[cfg(target_arch = "wasm32")]
pub fn get_current_url() -> String {
    let window = window().expect("should have a Window");
    let location = window.location();
    location.href().unwrap() // Directly unwrap the Result
}

#[cfg(target_arch = "wasm32")]
fn extract_until_question_mark(s: &str) -> &str {
    // Split the string at the first occurrence of '?'
    // and return the part before it.
    // If '?' is not found, return the entire string.
    s.split_once('?').map_or(s, |(before, _)| before)
}

#[cfg(target_arch = "wasm32")]
fn navigate_to_globe(globe_id: &str) {
    if let Some(window) = window() {
        let current_url = get_current_url();
        let base_url = extract_until_question_mark(&current_url);
        let full_url = format!("{}?globe={}", base_url, globe_id); // Append the globe_id as a query parameter.

        let location = window.location();
        match location.set_href(&full_url) {
            Ok(_) => {},
            Err(e) => eprintln!("Error setting the URL: {:?}", e),
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xAD as f32 / 255.0,
            0xD8 as f32 / 255.0,
            0xE6 as f32 / 255.0,
        )))
        .insert_resource(ApiURL(get_api_url()))
        .add_state::<AppState>()
        //.insert_resource(WinitSettings::desktop_app())
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            )
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(GlobePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(OrbitCameraControllerPlugin)
        .add_plugins(QueryServerPlugin)
        .add_plugins(GridMenuPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, update_directional_light_direction)
        .run();
}


#[derive(Resource)]
pub struct ApiURL(pub String);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    EditUpsert,
    EditUpsertSetSpeed,
    EditDelete,
    Orbiting,
    Zooming,
}

#[derive(Component)]
pub struct FollowingCamera;


fn setup_graphics(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        ..default()
    }).insert(FollowingCamera);
    
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::GRAY,
        brightness: 0.3,
    });
}

pub fn setup_physics(
    mut rapier_config: ResMut<RapierConfiguration>,) {
    rapier_config.gravity = Vec3::new(0.0, 0.0, 0.0);
}

fn update_directional_light_direction(
    //camera_query: Query<&Transform, With<Camera>>,
    camera_query: Query<(&Camera, &Transform), Without<FollowingCamera>>,
    mut light_query: Query<&mut Transform, With<FollowingCamera>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        for mut light_transform in light_query.iter_mut() {
            // Create a quaternion for a rotation about the x-axis
            let x_rotation = Quat::from_rotation_x(-PI / 4.);

            // Create a quaternion for a rotation about the y-axis
            let y_rotation = Quat::from_rotation_y(-PI / 4.);

            // Combine the rotations
            let combined_rotation = x_rotation * y_rotation;

            light_transform.rotation = camera_transform.1.rotation * combined_rotation;
        }
    }
}
