use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use super::spawn::*;
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::render::texture::Image;
use qrcode::QrCode;
use image::Rgba;

pub fn check_cursor_over_ui(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut touch_events: EventReader<TouchInput>,
    mut query: Query<(&mut Visibility, &Node, &GlobalTransform), With<Menu>>,
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
    query: &mut Query<(&mut Visibility, &Node, &GlobalTransform), With<Menu>>,
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
    for touch in touch_events.read() {
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
    for touch in touch_events.read() {
        if touch.phase == TouchPhase::Started {
            for (entity, global_transform, node) in touch_input_query.iter() {
                if is_touch_over_button(touch, global_transform, node) {
                    toggle_delete_button(&mut commands, &mut selected_query, entity, &mut selected_delete);
                }
            }
        }
    }
}

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

pub fn create_new_globe_button_selector(
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<CreateNewGlobeButton>)>,
    touch_input_query: Query<(Entity, &GlobalTransform, &Node), With<CreateNewGlobeButton>>,
    mut touch_events: EventReader<TouchInput>,
    mut send_create_new_globe_event: EventWriter<crate::query_server::SendCreateNewGlobeEvent>,
) {
    // Handle mouse interaction
    for (_entity, interaction) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            send_create_new_globe_event.send(crate::query_server::SendCreateNewGlobeEvent);
        }
    }

    // Handle touch events
    for touch in touch_events.read() {
        if touch.phase == TouchPhase::Started {
            for (_entity, global_transform, node) in touch_input_query.iter() {
                if is_touch_over_button(touch, global_transform, node) {
                    send_create_new_globe_event.send(crate::query_server::SendCreateNewGlobeEvent);
                }
            }
        }
    }
}

pub fn info_button_selector(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<InfoButton>)>,
    touch_input_query: Query<(Entity, &GlobalTransform, &Node), With<InfoButton>>,
    mut touch_events: EventReader<TouchInput>,
    mut selected_query: Query<Entity, (With<SelectedInfoButton>, With<InfoButton>)>,
    mut selected_info: ResMut<SelectedInfo>,
    mut query_info_panel: Query<&mut Visibility, With<InfoPanel>>,
    mut image_resources: ResMut<ImageResources>,
    mut images: ResMut<Assets<Image>>,
    mut query_qr_button_image: Query<&mut UiImage, With<QRButtonImage>>,
    mut query_qr_button_text: Query<&mut Text, With<QRButtonText>>,
) {
    // Handle mouse interaction
    for (entity, interaction) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            toggle_info_button(&mut commands, &mut selected_query, entity, &mut selected_info, &mut query_info_panel, &mut image_resources, &mut images, &mut query_qr_button_image, &mut query_qr_button_text);
        }
    }

    // Handle touch events
    for touch in touch_events.read() {
        if touch.phase == TouchPhase::Started {
            for (entity, global_transform, node) in touch_input_query.iter() {
                if is_touch_over_button(touch, global_transform, node) {
                    toggle_info_button(&mut commands, &mut selected_query, entity, &mut selected_info, &mut query_info_panel, &mut image_resources, &mut images, &mut query_qr_button_image, &mut query_qr_button_text);
                }
            }
        }
    }
}

fn toggle_info_button(
    commands: &mut Commands,
    selected_query: &mut Query<Entity, (With<SelectedInfoButton>, With<InfoButton>)>,
    entity: Entity,
    selected_info: &mut ResMut<SelectedInfo>,
    query_info_panel: &mut Query<&mut Visibility, With<InfoPanel>>,
    image_resources: &mut ResMut<ImageResources>,
    images: &mut ResMut<Assets<Image>>,
    query_qr_button_image: &mut Query<&mut UiImage, With<QRButtonImage>>,
    query_qr_button_text: &mut Query<&mut Text, With<QRButtonText>>,
) {
    if let Ok(previous_entity) = selected_query.get_single_mut() {
        commands.entity(previous_entity).remove::<SelectedInfoButton>();
        selected_info.0 = false;
        //Hide info panel
        for mut visibility in query_info_panel.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    } else {
        commands.entity(entity).insert(SelectedInfoButton);
        selected_info.0 = true;
        //Show info panel
        for mut visibility in query_info_panel.iter_mut() {
            *visibility = Visibility::Visible;
        }
        //Generate correct QRImage for current url
        let url = crate::get_current_url();
        let qr_code = QrCode::new(url.clone()).unwrap();

        let qr_image = qr_code.render::<Rgba<u8>>()
        .min_dimensions(200, 200)
        .build();
    
        // Convert the QR code image (image crate) to Bevy texture
        let bevy_image = Image::new_fill(
            Extent3d {
                width: qr_image.width(),
                height: qr_image.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &qr_image.into_raw(),
            TextureFormat::Rgba8Unorm, // Assuming conversion to RGBA is done if necessary
            default()
        );

        // Replace the old image with the new one
        let handle = images.add(bevy_image);

        // Remove the old image from the assets (if it's not the default handle)
        if image_resources.qr != Handle::<Image>::default() {
            images.remove(image_resources.qr.clone());
        }

        // Update the handle to the new image
        for mut image in query_qr_button_image.iter_mut() {
            image.texture = handle.clone();
        }

        // Change text
        for mut text in query_qr_button_text.iter_mut() {
            text.sections[0].value = url.clone();
        }
    }
}


pub fn update_info_button_appearance(
    mut query: Query<(&mut Style, Option<&SelectedInfoButton>), With<InfoButton>>,
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
