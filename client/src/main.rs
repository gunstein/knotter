use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy::input::mouse::{MouseButtonInput, MouseMotion};

use globe::GlobePlugin;
use ball::BallPlugin;
use orbit_camera_controller::OrbitCameraControllerPlugin;
use query_server::QueryServerPlugin;

use std::f32::consts::PI;

mod globe;
mod ball;
mod query_server;
mod orbit_camera_controller;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_state::<AppState>()
        //.insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        //.add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(DefaultPickingPlugins)
        //.add_plugins(LookTransformPlugin)
        //.add_plugins(OrbitCameraPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(GlobePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(OrbitCameraControllerPlugin)
        //.add_plugins(UiPlugin)
        .add_plugins(QueryServerPlugin)
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        //.add_systems(Startup, setup_spotlights)
        .add_systems(Update, update_directional_light_direction)
        .run();
}


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Upsert,
    UpsertSetSpeed,
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
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
