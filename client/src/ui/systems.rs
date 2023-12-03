use bevy::prelude::*;
use super::spawn::ColorAndDeleteMenu;

pub fn check_mouse_over_ui_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<(&mut Visibility, &Node, &GlobalTransform), With<ColorAndDeleteMenu>>,
) {
    for event in cursor_moved_events.read() {
        for (mut visibility, node, global_transform) in query.iter_mut() {            
            let cursor_pos_ui = event.position;

            let node_pos = global_transform.translation().truncate();

            // Check if the cursor is over the UI element
            if (cursor_pos_ui.x > node_pos.x - node.size().x / 2.0) && (cursor_pos_ui.x < node_pos.x + node.size().x / 2.0) &&
            (cursor_pos_ui.y > node_pos.y - node.size().y / 2.0) && (cursor_pos_ui.y < node_pos.y + node.size().y / 2.0) {
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}