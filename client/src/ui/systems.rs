use bevy::prelude::*;
use super::spawn::*;
use bevy::input::touch::{TouchInput, TouchPhase};

pub fn check_cursor_over_ui_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut touch_events: EventReader<TouchInput>,
    mut query: Query<(&mut Visibility, &Node, &GlobalTransform), With<ColorAndDeleteMenu>>,
) {
    // Handle cursor movements
    for event in cursor_moved_events.read() {
        update_visibility_based_on_position(event.position, &mut query);
    }

    // Handle touch events
    for touch in touch_events.read() {
        if touch.phase == TouchPhase::Started || touch.phase == TouchPhase::Moved {
            let touch_pos_ui = touch.position;
            update_visibility_based_on_position(touch_pos_ui, &mut query);
        }
    }
}

fn update_visibility_based_on_position(
    position: Vec2,
    query: &mut Query<(&mut Visibility, &Node, &GlobalTransform), With<ColorAndDeleteMenu>>,
) {
    for (mut visibility, node, global_transform) in query.iter_mut() {
        let node_pos = global_transform.translation().truncate();

        // Check if the position is over the UI element
        if (position.x > node_pos.x - node.size().x / 2.0) && (position.x < node_pos.x + node.size().x / 2.0) &&
           (position.y > node_pos.y - node.size().y / 2.0) && (position.y < node_pos.y + node.size().y / 2.0) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}


pub fn color_button_selector(
    mut commands: Commands,
    interaction_query: Query<(Entity, &ColorButton, &Interaction), (Changed<Interaction>, With<Button>)>,
    touch_input_query: Query<(Entity, &ColorButton, &GlobalTransform, &Node), With<Button>>,
    mut touch_events: EventReader<TouchInput>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), (With<SelectedColorButton>, With<ColorButton>)>,
    mut selected_color: ResMut<SelectedColor>,
) {
    // Handle mouse interaction
    for (entity, color_button, interaction) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            select_color_button(&mut commands, &mut selected_query, entity, color_button, &mut selected_color);
        }
    }

    // Handle touch events
    for touch in touch_events.iter() {
        if touch.phase == TouchPhase::Started {
            for (entity, color_button, global_transform, node) in touch_input_query.iter() {
                if is_touch_over_button(touch, global_transform, node) {
                    select_color_button(&mut commands, &mut selected_query, entity, color_button, &mut selected_color);
                }
            }
        }
    }
}

fn is_touch_over_button(touch: &TouchInput, global_transform: &GlobalTransform, node: &Node) -> bool {
    let node_pos = global_transform.translation().truncate();
    let touch_pos_ui = touch.position;

    // Check if the touch is over the button
    (touch_pos_ui.x > node_pos.x - node.size().x / 2.0) && (touch_pos_ui.x < node_pos.x + node.size().x / 2.0) &&
    (touch_pos_ui.y > node_pos.y - node.size().y / 2.0) && (touch_pos_ui.y < node_pos.y + node.size().y / 2.0)
}

fn select_color_button(
    commands: &mut Commands,
    selected_query: &mut Query<(Entity, &mut BackgroundColor), (With<SelectedColorButton>, With<ColorButton>)>,
    entity: Entity,
    color_button: &ColorButton,
    selected_color: &mut ResMut<SelectedColor>,
) {
    if let Ok((previous_entity, mut previous_color)) = selected_query.get_single_mut() {
        *previous_color = BackgroundColor(color_button.0); // Revert color of previously selected
        commands.entity(previous_entity).remove::<SelectedColorButton>();
    }
    commands.entity(entity).insert(SelectedColorButton);
    selected_color.0 = color_button.0;
}



/* 
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
*/

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


pub fn delete_button_selector(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<DeleteButton>)>,
    touch_input_query: Query<(Entity, &GlobalTransform, &Node), With<DeleteButton>>,
    mut touch_events: EventReader<TouchInput>,
    mut selected_query: Query<Entity, (With<SelectedDeleteButton>, With<DeleteButton>)>,
    mut selected_delete: ResMut<SelectedDelete>,
) {
    // Handle mouse interaction
    for (entity, interaction) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            toggle_delete_button(&mut commands, &mut selected_query, entity, &mut selected_delete);
        }
    }

    // Handle touch events
    for touch in touch_events.iter() {
        if touch.phase == TouchPhase::Started {
            for (entity, global_transform, node) in touch_input_query.iter() {
                if is_touch_over_button(touch, global_transform, node) {
                    toggle_delete_button(&mut commands, &mut selected_query, entity, &mut selected_delete);
                }
            }
        }
    }
}

/*fn is_touch_over_button(touch: &TouchInput, global_transform: &GlobalTransform, node: &Node) -> bool {
    let node_pos = global_transform.translation().truncate();
    let touch_pos_ui = touch.position;

    // Check if the touch is over the button
    (touch_pos_ui.x > node_pos.x - node.size().x / 2.0) && (touch_pos_ui.x < node_pos.x + node.size().x / 2.0) &&
    (touch_pos_ui.y > node_pos.y - node.size().y / 2.0) && (touch_pos_ui.y < node_pos.y + node.size().y / 2.0)
}
*/

fn toggle_delete_button(
    commands: &mut Commands,
    selected_query: &mut Query<Entity, (With<SelectedDeleteButton>, With<DeleteButton>)>,
    entity: Entity,
    selected_delete: &mut ResMut<SelectedDelete>,
) {
    if let Ok(previous_entity) = selected_query.get_single_mut() {
        commands.entity(previous_entity).remove::<SelectedDeleteButton>();
        selected_delete.0 = false;
    } else {
        commands.entity(entity).insert(SelectedDeleteButton);
        selected_delete.0 = true;
    }
}

/* 
pub fn delete_button_selector(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<DeleteButton>)>,
    mut selected_query: Query<Entity, (With<SelectedDeleteButton>, With<DeleteButton>)>,
    mut selected_delete: ResMut<SelectedDelete>,
) {
    //bevy::log::info!("color_button_selector 0");
    for (entity, interaction) in interaction_query.iter() {
        //bevy::log::info!("color_button_selector 1");
        if *interaction == Interaction::Pressed {
            if let Ok(previous_entity) = selected_query.get_single_mut() {
                commands.entity(previous_entity).remove::<SelectedDeleteButton>();
                selected_delete.0 = false;
            }
            else{
                commands.entity(entity).insert(SelectedDeleteButton);
                selected_delete.0 = true;
            }
        }
    }
}
*/

pub fn update_delete_button_appearance(
    mut query: Query<(&mut Style, Option<&SelectedDeleteButton>), With<DeleteButton>>,
) {
    for ( mut style, selected) in query.iter_mut() {
        if let Some(_) = selected {
            // Change appearance to indicate selection
            style.margin = UiRect::all(Val::Px(3.0));
        } else {
            // Revert to normal appearance
            style.margin = UiRect::all(Val::Px(0.0));
        }
    }
}