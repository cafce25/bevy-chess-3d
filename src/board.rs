use bevy::{
    app::{AppExit, Events},
    prelude::*,
};
use bevy_mod_picking::{Hover, PickableBundle, PickingEvent, Selection};

use crate::pieces::{Piece, PieceColor, PieceType};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPiece>()
            .init_resource::<PlayerTurn>()
            .init_resource::<SquareMaterials>()
            .add_event::<ResetSelectedEvent>()
            .add_startup_system(create_board)
            .add_system(color_squares)
            .add_system(move_piece)
            .add_system(select_piece)
            .add_system(despawn_taken_pieces)
            .add_system(reset_selected);
    }
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}

pub struct PlayerTurn(pub PieceColor);

impl PlayerTurn {
    fn change(&mut self) {
        self.0 = self.0.opponent();
    }
}
impl Default for PlayerTurn {
    fn default() -> Self {
        Self(PieceColor::Light)
    }
}

fn color_squares(
    materials: ResMut<SquareMaterials>,
    mut query: Query<(&Square, &Selection, &Hover, &mut Handle<StandardMaterial>)>,
) {
    for (square, selection, hover, mut material) in query.iter_mut() {
        // Change the material
        *material = if hover.hovered() {
            materials.highlight_color.clone()
        } else if selection.selected() {
            materials.selected_color.clone()
        } else if square.is_light() {
            materials.light_color.clone()
        } else {
            materials.dark_color.clone()
        };
    }
}

fn select_piece(
    mut events: EventReader<PickingEvent>,
    mut selected_piece: ResMut<SelectedPiece>,
    turn: Res<PlayerTurn>,
    squares: Query<(&Square, &Selection)>,
    pieces: Query<(Entity, &Piece)>,
) {
    if !events
        .iter()
        .any(|e| matches!(e, PickingEvent::Selection(_)))
    {
        return;
    }
    let square = if let Some(square) = squares
        .iter()
        .find_map(|(square, selection)| selection.selected().then(|| square))
    {
        square
    } else {
        return;
    };

    if selected_piece.entity.is_none() {
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

fn move_piece(
    mut commands: Commands,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
    squares: Query<(&Square, &Selection)>,
    mut pieces: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: ResMut<Events<ResetSelectedEvent>>,
) {
    let square = if let Some(square) = squares
        .iter()
        .find_map(|(square, selection)| selection.selected().then(|| square))
    {
        square
    } else {
        return;
    };

    if let Some(selected_piece_entity) = selected_piece.entity {
        let pieces_vec = pieces.iter().map(|(_, piece)| *piece).collect();
        // collect is needed here to satisfy the borrow checker
        #[allow(clippy::needless_collect)]
        let pieces_entity_vec = pieces
            .iter()
            .map(|(entity, piece)| (entity, *piece))
            .collect::<Vec<_>>();
        // Move the selected piece to the selected square
        let mut piece = if let Ok((_piece_entity, piece)) = pieces.get_mut(selected_piece_entity) {
            piece
        } else {
            return;
        };

        if piece.is_move_valid((square.x, square.y), pieces_vec) {
            // Check if a piece of the opposite color exists in this square and mark it for
            // despawning
            if let Some((other_entity, _other_piece)) = pieces_entity_vec
                .into_iter()
                .find(|(_, target_piece)| target_piece.x == square.x && target_piece.y == square.y)
            {
                // Mark the piece as taken
                commands.entity(other_entity).insert(Taken);
            }

            // Move piece
            piece.x = square.x;
            piece.y = square.y;

            // Change turn
            turn.change();

            reset_selected_event.send(ResetSelectedEvent);
        }
    }
}

fn reset_selected(
    mut event_reader: EventReader<ResetSelectedEvent>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut square_selections: Query<&mut Selection, With<Square>>,
) {
    for _event in event_reader.iter() {
        selected_piece.entity = None;
        for mut selection in square_selections.iter_mut() {
            selection.set_selected(false);
        }
    }
}

fn despawn_taken_pieces(
    mut commands: Commands,
    mut app_exit_events: ResMut<Events<AppExit>>,
    query: Query<(Entity, &Piece), With<Taken>>,
) {
    for (entity, piece) in query.iter() {
        // TODO: detect check and mate
        // If the king is taken, we should exit

        if piece.piece_type == PieceType::King {
            println!("{} won! Thanks for playing!", piece.color.opponent());
            app_exit_events.send(AppExit);
        }

        commands.entity(entity).despawn_recursive();
    }
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<SquareMaterials>,
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
                        materials.light_color.clone()
                    } else {
                        materials.dark_color.clone()
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

struct ResetSelectedEvent;
#[derive(Component)]
struct Taken;

struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    dark_color: Handle<StandardMaterial>,
    light_color: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
    fn from_world(world: &mut World) -> Self {

        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
            //resoucrses.get_mut::<Assets<StandardMaterial>>().unwrap();
        SquareMaterials {
            highlight_color: materials.add(Color::rgb(0.8, 0.3, 0.3).into()),
            selected_color: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
            dark_color: materials.add(Color::rgb(0.0, 0.1, 0.1).into()),
            light_color: materials.add(Color::rgb(1.0, 0.9, 0.9).into()),
        }
    }
}

