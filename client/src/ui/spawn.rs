use bevy::prelude::*;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct ColorButton(pub Color); // Represents the color of each button

#[derive(Resource)]
pub struct SelectedColor(pub Color); // Stores the currently selected color

#[derive(Component)]
pub struct SelectedColorButton;

#[derive(Component)]
pub struct DeleteButton; 

#[derive(Component)]
pub struct SelectedDeleteButton;

#[derive(Resource)]
pub struct SelectedDelete(pub bool);

#[derive(Component)]
pub struct CreateNewGlobeButton; 

#[derive(Component)]
pub struct InfoButton; 

#[derive(Component)]
pub struct InfoPanel; 

#[derive(Component)]
pub struct SelectedInfoButton;

#[derive(Resource)]
pub struct SelectedInfo(pub bool);

#[derive(Component)]
pub struct QRButton; 

#[derive(Component)]
pub struct QRButtonImage; 

#[derive(Component)]
pub struct QRButtonText; 

#[derive(Resource)]
pub struct ImageResources {
    pub delete_ball: Handle<Image>,
    pub info: Handle<Image>,
    pub plus: Handle<Image>,
    pub qr: Handle<Image>,
}

impl Default for ImageResources {
    fn default() -> Self {
        ImageResources {
            delete_ball: Handle::default(),
            info: Handle::default(),
            plus: Handle::default(),
            qr: Handle::default(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum ButtonType {
    DeleteButton,
    CreateButton,
    InfoButton,
    QRButton,
}

pub fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>, 
    mut image_resources: ResMut<ImageResources>,
    //color_material_map: Res<ColorMaterialMap>
) {
    image_resources.delete_ball = asset_server.load("delete_ball.png");
    image_resources.plus = asset_server.load("plus.png");
    image_resources.info = asset_server.load("info.png");

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Extract colors from the ColorMaterialMap keys
    //let colors: Vec<Color> = color_material_map.map.keys().map(|color_key| color_key.0).collect();

    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_columns: vec![GridTrack::percent(20.), GridTrack::percent(60.), GridTrack::percent(20.)],
                grid_template_rows: vec![
                    GridTrack::percent(25.),
                    GridTrack::percent(25.),
                    GridTrack::percent(25.),
                    GridTrack::percent(25.),
                ],
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            //Left column
            builder
                .spawn(NodeBundle {
                    style: Style {
                        grid_row: GridPlacement::span(2),
                        // Make the height of the node fill its parent
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(24.0)),
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                })
                .with_children(|builder| {
                    item_rect_color(builder, Color::ORANGE, false);
                    item_rect_color(builder, Color::BISQUE, false);
                    item_rect_color(builder, Color::BLUE, true);

                    item_rect_color(builder, Color::CYAN, false);
                    item_rect_color(builder, Color::ORANGE_RED, false);
                    item_rect_color(builder, Color::DARK_GREEN, false);

                    item_rect_color(builder, Color::TEAL, false);
                    item_rect_color(builder, Color::ALICE_BLUE, false);
                    //item_rect_image(builder, image_resources.delete_ball.clone(), ButtonType::DeleteButton);
                })
                .insert(Menu);

            // Middle column
            builder
                .spawn(NodeBundle {
                    style: Style {
                        grid_row: GridPlacement::span(4),
                        ..default()
                    },
                    //background_color: BackgroundColor(Color::BLACK),
                    visibility: Visibility::Hidden,
                    ..default()
                });

            // Right column
            builder
                .spawn(NodeBundle {
                    style: Style {
                        grid_row: GridPlacement::span(2),
                        // Make the height of the node fill its parent
                        width: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(24.0)),
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 2 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 2 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(3, 1.0),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                })
                .with_children(|builder| {
                    item_rect_image(builder, image_resources.plus.clone(), ButtonType::CreateButton);
                    item_rect_image(builder, image_resources.info.clone(), ButtonType::InfoButton);
                    item_rect_image(builder, image_resources.delete_ball.clone(), ButtonType::DeleteButton);
                })
                .insert(Menu);

            // Left bottom
            builder.spawn(NodeBundle {
                style: Style {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_row: GridPlacement::span(2),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                visibility: Visibility::Hidden,
                ..default()
            });

            // Rigth bottom
            builder
                .spawn(NodeBundle {
                    style: Style {
                        grid_row: GridPlacement::span(2),
                        // Make the height of the node fill its parent
                        width: Val::Percent(100.0),
                        //aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        //padding: UiRect::all(Val::Px(24.0)),
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 2 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 2 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                })
                .with_children(|builder| {
                    // QR Code Image
                    item_rect_image(builder, image_resources.delete_ball.clone(), ButtonType::QRButton);
        
                    // URL Text Box
                    builder.spawn(TextBundle::from_section(
                        "Testing",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                    ))
                    .insert(QRButtonText);
                
                    builder.spawn(NodeBundle::default());
                })
                .insert(InfoPanel);
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
fn item_rect_color(builder: &mut ChildBuilder, color: Color, is_selected: bool) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            let mut button = builder.spawn(ButtonBundle {
                //background_color: BackgroundColor(color),
                background_color: BackgroundColor(color),
                ..default()
            });
            button.insert(ColorButton(color));
            if is_selected{
                button.insert(SelectedColorButton);
            }
        });
}

fn item_rect_image(
    builder: &mut ChildBuilder, 
    image: Handle<Image>, 
    button_type: ButtonType,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            let mut button = builder.spawn(ButtonBundle {
                style: Style { 
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });

            // Add the image to the button
            button.with_children(|parent| {
                let mut image = parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    image: image.into(),
                    ..default()
                });
                if button_type == ButtonType::QRButton {
                    image.insert(QRButtonImage);
                }
            });

            // Insert the specific component based on the button type
            match button_type {
                ButtonType::DeleteButton => {
                    button.insert(DeleteButton);
                },
                ButtonType::CreateButton => {
                    button.insert(CreateNewGlobeButton);
                },
                ButtonType::InfoButton => {
                    button.insert(InfoButton);
                },
                ButtonType::QRButton => {
                    button.insert(QRButton);
                },
            }
        });
}
