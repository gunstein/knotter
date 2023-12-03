use bevy::prelude::*;

#[derive(Component)]
pub struct ColorAndDeleteMenu;

pub fn spawn_layout(mut commands: Commands) {
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
                    item_rect(builder, Color::ORANGE);
                    item_rect(builder, Color::BISQUE);
                    item_rect(builder, Color::BLUE);

                    item_rect(builder, Color::CYAN);
                    item_rect(builder, Color::ORANGE_RED);
                    item_rect(builder, Color::DARK_GREEN);

                    item_rect(builder, Color::TEAL);
                    item_rect(builder, Color::ALICE_BLUE);
                    item_rect(builder, Color::CRIMSON);
                })
                .insert(ColorAndDeleteMenu);

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
                        grid_row: GridPlacement::span(4),
                        ..default()
                    },
                    //background_color: BackgroundColor(Color::BLACK),
                    visibility: Visibility::Hidden,
                    ..default()
                });

            // Left bottom
            builder.spawn(NodeBundle {
                style: Style {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_row: GridPlacement::span(2),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            });
        });
}

/// Create a coloured rectangle node. The node has size as it is assumed that it will be
/// spawned as a child of a Grid container with `AlignItems::Stretch` and `JustifyItems::Stretch`
/// which will allow it to take it's size from the size of the grid area it occupies.
fn item_rect(builder: &mut ChildBuilder, color: Color) {
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
            builder.spawn(NodeBundle {
                background_color: BackgroundColor(color),
                ..default()
            });
        });
}