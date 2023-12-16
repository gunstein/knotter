use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

use globe::GlobePlugin;
use ball::BallPlugin;
use orbit_camera_controller::OrbitCameraControllerPlugin;
use query_server::QueryServerPlugin;
use ui::GridMenuPlugin;
use std::path::Path;

use std::f32::consts::PI;

mod globe;
mod ball;
mod query_server;
mod orbit_camera_controller;
mod ui;

//use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn get_url_path() -> String;
}

#[cfg(not(target_arch = "wasm32"))]
fn get_url_path() -> String {
    "gvtest123".to_string()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn get_api_url() -> String;
}
#[cfg(not(target_arch = "wasm32"))]
fn get_api_url() -> String {
    "http://192.168.86.166:8080".to_string()
}

fn main() {
    //let path = get_url_path();
    //println!("URL Path: {}", path);

    //let api_url = get_api_url();
    //println!("API URL: {}", api_url);
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xAD as f32 / 255.0,
            0xD8 as f32 / 255.0,
            0xE6 as f32 / 255.0,
        )))
        .insert_resource(GlobeName(first_path_element(&get_url_path()).unwrap()))
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

fn first_path_element(path_str: &str) -> Option<String> {
    Path::new(path_str)
        .components()
        .find_map(|component| match component {
            std::path::Component::Normal(c) => c.to_str().map(String::from),
            _ => None,
        })
}

#[derive(Resource)]
pub struct GlobeName(pub String);

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
