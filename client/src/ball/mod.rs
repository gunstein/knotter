use bevy::prelude::*;

use crate::AppState;

pub mod components;
pub mod resources;
pub mod systems;
pub mod spawn;
pub mod color_material_map;

use systems::*;
use color_material_map::*;
use std::collections::HashMap;

pub const BALL_RADIUS: f32 = 0.05;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, spawn_static_balls)
            .insert_resource(ColorMaterialMap {
                map: HashMap::new(),
            })
            .add_systems(PreStartup, init_ball_resources)
            //.add_systems(Startup, spawn_moving_balls)
            .add_systems(Update, push_ball_against_globe)
            .add_systems(Update, handle_ball_collision)
            .add_systems(Update, handle_delete_state.run_if(in_state(AppState::EditUpsert)))
            .add_systems(Update, handle_delete_state.run_if(in_state(AppState::EditDelete)))
            .add_systems(Update, edit_upsert_ball_on_globe.run_if(in_state(AppState::EditUpsert)))
            .add_systems(Update, edit_upsert_set_speed.run_if(in_state(AppState::EditUpsertSetSpeed)))
            .add_systems(Update, finalize_upsert_ball_on_globe.run_if(in_state(AppState::EditUpsertSetSpeed)))
            .add_systems(Update, edit_delete_ball.run_if(in_state(AppState::EditDelete)))
            .add_systems(Update, receive_ball_transactions_event_listener);
        
    }
}
