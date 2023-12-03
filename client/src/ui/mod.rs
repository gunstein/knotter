use bevy::prelude::*;

pub mod systems;
pub mod spawn;

use systems::*;
use spawn::*;

pub struct GridMenuPlugin;

impl Plugin for GridMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_layout)
            .add_systems(Update, check_mouse_over_ui_system);
    }
}