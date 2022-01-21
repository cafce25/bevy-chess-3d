use bevy::prelude::*;
use bevy_mod_picking::{Hover, PickableBundle, Selection};

use crate::pieces::{Piece, PieceColor};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .add_startup_system(create_board)
            .add_system(color_squares)
            .add_system(select_square);
    }
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

struct PlayerTurn(PieceColor);
impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::Light)
    }
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
    mut turn: ResMut<PlayerTurn>,
    mut squares: Query<(&Square, &mut Selection)>,
    mut pieces: Query<(Entity, &mut Piece)>,
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the selected square
    if let Some((square, mut selection)) = squares
        .iter_mut()
        .find(|(_, selection)| selection.selected())
    {
        if let Some(selected_piece_entity) = selected_piece.entity {
            let pieces_entity_vec: Vec<(Entity, Piece)> = pieces
                .iter()
                .map(|(entity, piece)| (entity, *piece))
                .collect();
            let pieces_vec = pieces.iter().map(|(_, piece)| *piece).collect();
            // Move the selected piece to the selected square
            if let Ok((_entity, mut piece)) = pieces.get_mut(selected_piece_entity) {
                if piece.is_move_valid((square.x, square.y), pieces_vec) {
                    if let Some((entity, _piece)) =
                        pieces_entity_vec.into_iter().find(|(_, target_piece)| {
                            target_piece.x == square.x && target_piece.y == square.y
                        })
                    {
                        // Despawn piece and it's children
                        commands.entity(entity).despawn_recursive();
                    }
                    piece.x = square.x;
                    piece.y = square.y;

                    turn.0 = match turn.0 {
                        PieceColor::Dark => PieceColor::Light,
                        PieceColor::Light => PieceColor::Dark,
                    }
                }
            }
            selected_piece.entity = None;
            selection.set_selected(false);
        } else {
            // Select the piece in the currently selected square
            for (piece_entity, piece) in pieces.iter() {
                if piece.x == square.x && piece.y == square.y && piece.color == turn.0 {
                    // piece_entity is now the entity in the same square
                    selected_piece.entity = Some(piece_entity);
                    break;
                }
            }
        }
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));

    // Spawn 64 squares {
    for x in 0..8 {
        for y in 0..8 {
            let square = Square { x, y };
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: if square.is_light() {
                        materials.add(Color::rgb(1.0, 0.9, 0.9).into())
                    } else {
                        materials.add(Color::rgb(0.0, 0.1, 0.1).into())
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, 0.0, y as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(square);
        }
    }
}

#[derive(Component)]
struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub fn is_light(&self) -> bool {
        (self.x + self.y) % 2 == 1
    }
}
