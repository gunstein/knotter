use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, Smoother};

use bevy::input::mouse::{MouseButtonInput, MouseMotion};

use bevy_mod_picking::prelude::*;

use globe::GlobePlugin;
use ball::BallPlugin;
use query_server::QueryServerPlugin;

use std::f32::consts::PI;

mod globe;
mod ball;
mod query_server;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        //.insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(GlobePlugin)
        .add_plugins(BallPlugin)
        .add_plugins(QueryServerPlugin)
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });
  
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
    
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    commands
    .spawn(Camera3dBundle::default())
    .insert(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        Vec3::new(0.0, 6.0, 0.0),
        Vec3::new(0., 0., 0.),
        Vec3::Y,
    ))
    .insert(RaycastPickCamera::default());
}

pub fn setup_physics(
    mut rapier_config: ResMut<RapierConfiguration>,) {
    rapier_config.gravity = Vec3::new(0.0, 0.0, 0.0);
}

