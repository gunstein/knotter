use bevy::prelude::*;

use crate::AppState;

pub mod components;
pub mod resources;
pub mod systems;
pub mod spawn;

use systems::*;

pub const BALL_RADIUS: f32 = 0.05;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_systems(Startup, spawn_static_balls)
            .add_systems(Startup, init_ball_resources)
            //.add_systems(Startup, spawn_moving_balls)
            .add_systems(Update, push_ball_against_globe)
            .add_systems(Update, handle_ball_collision)
            .add_systems(Update, edit_upsert_ball_on_globe.run_if(in_state(AppState::EditUpsert)))
            .add_systems(Update, edit_upsert_set_speed.run_if(in_state(AppState::EditUpsertSetSpeed)))
            .add_systems(Update, finalize_upsert_ball_on_globe.run_if(in_state(AppState::EditUpsertSetSpeed)));
    }
}
