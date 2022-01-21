use bevy::prelude::*;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DefaultPickingPlugins, Hover, PickingCameraBundle, Selection,
};
use board::Square;
use pieces::{Piece, PiecesPlugin};

mod board;
mod pieces;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(PiecesPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .init_resource::<SelectedPiece>()
        .add_startup_system(setup)
        .add_startup_system(board::create_board)
        .add_system(color_squares)
        .add_system(select_square)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default());
    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn color_squares(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Square, &Selection, &Hover, &Handle<StandardMaterial>)>,
) {
    for (square, selection, hover, material_handle) in query.iter() {
        // Get the actual material
        let material = materials.get_mut(material_handle).unwrap();

        // Change the material color
        material.base_color = if hover.hovered() {
            Color::rgb(0.7, 0.3, 0.3)
        } else if selection.selected() {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_light() {
            Color::rgb(1.0, 0.9, 0.9)
        } else {
            Color::rgb(0.0, 0.1, 0.1)
        };
    }
}

fn select_square(
    mut commands: Commands,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares: Query<(&Square, &Selection)>,
    mut pieces: Query<(Entity, &mut Piece)>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the selected square
    let selected_square = squares
        .iter()
        .find_map(|(square, selection)| selection.selected().then(|| square));
    if let Some(square) = selected_square {
        if let Some(selected_piece_entity) = selected_piece.entity {
            let pieces_entity_vec: Vec<(Entity, Piece)> = pieces
                .iter()
                .map(|(entity, piece)| (entity, *piece))
                .collect();
            let pieces_vec = pieces.iter().map(|(_, piece)| *piece).collect();
            // Move the selected piece to the selected square
            if let Ok((_entity, mut piece)) = pieces.get_mut(selected_piece_entity) {
                if piece.is_move_valid((square.x, square.y), pieces_vec) {
                    if let Some((entity, _)) =
                        pieces_entity_vec.into_iter().find(|(_, target_piece)| {
                            target_piece.x == square.x && target_piece.y == square.y
                        })
                    {
                        // Despawn piece and it's children
                        commands.entity(entity).despawn_recursive();
                    }
                    piece.x = square.x;
                    piece.y = square.y;
                }
            }
            selected_piece.entity = None;
        } else {
            // Select the piece in the currently selected square
            for (piece_entity, piece) in pieces.iter() {
                if piece.x == square.x && piece.y == square.y {
                    // piece_entity is now the entity in the same square
                    selected_piece.entity = Some(piece_entity);
                    break;
                }
            }
        }
    }
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}
