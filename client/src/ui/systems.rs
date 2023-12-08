use bevy::prelude::*;
use super::spawn::*;

pub fn check_cursor_over_ui_system(
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

pub fn color_button_selector(
    mut commands: Commands,
    interaction_query: Query<(Entity, &ColorButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), (With<SelectedColorButton>, With<ColorButton>)>,
    mut selected_color: ResMut<SelectedColor>,
) {
    //bevy::log::info!("color_button_selector 0");
    for (entity, color_button, interaction) in interaction_query.iter() {
        //bevy::log::info!("color_button_selector 1");
        if *interaction == Interaction::Pressed {
            if let Ok((previous_entity, mut previous_color)) = selected_query.get_single_mut() {
                *previous_color = BackgroundColor(color_button.0); // Revert color of previously selected
                commands.entity(previous_entity).remove::<SelectedColorButton>();
            }
            commands.entity(entity).insert(SelectedColorButton);
            selected_color.0 = color_button.0;
        }
    }
}

pub fn update_color_button_appearance(
    mut query: Query<(&ColorButton, &mut BackgroundColor, &mut Style, Option<&SelectedColorButton>), With<Button>>,
) {
    for (color_button, mut ui_color, mut style, selected) in query.iter_mut() {
        if let Some(_) = selected {
            // Change appearance to indicate selection
            style.margin = UiRect::all(Val::Px(3.0));
            *ui_color = BackgroundColor(color_button.0 * 0.8); // Slightly darken the color to indicate selection
        } else {
            // Revert to normal appearance
            *ui_color = BackgroundColor(color_button.0); // Restore original color
            style.margin = UiRect::all(Val::Px(0.0));
        }
    }
}