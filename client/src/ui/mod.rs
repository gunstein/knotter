use bevy::prelude::*;

pub mod systems;
pub mod spawn;

use systems::*;
use spawn::*;

pub struct GridMenuPlugin;

impl Plugin for GridMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedColor(Color::BLUE))
            .insert_resource(SelectedDelete(false))
            .add_systems(Startup, spawn_layout)
            .add_systems(Update, check_cursor_over_ui_system)
            .add_systems(Update, color_button_selector)
            .add_systems(Update, update_color_button_appearance)
            .add_systems(Update, delete_button_selector)
            .add_systems(Update, update_delete_button_appearance);
    }
}