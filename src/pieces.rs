use bevy::prelude::*;

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces)
            .add_system(move_pieces);
    }
}

fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load all the meshes
    let king_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> =
        asset_server.load("models/chess/kit/pieces.glb#Mesh7/Primitive0");

    // Add some materials
    let light_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let dark_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());

    spawn_rook(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        rook_handle.clone(),
        (0, 0),
    );
    spawn_knight(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 1),
    );
    spawn_bishop(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        bishop_handle.clone(),
        (0, 2),
    );
    spawn_queen(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        queen_handle.clone(),
        (0, 3),
    );
    spawn_king(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        king_handle.clone(),
        king_cross_handle.clone(),
        (0, 4),
    );
    spawn_bishop(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        bishop_handle.clone(),
        (0, 5),
    );
    spawn_knight(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 6),
    );
    spawn_rook(
        &mut commands,
        light_material.clone(),
        PieceColor::Light,
        rook_handle.clone(),
        (0, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            light_material.clone(),
            PieceColor::Light,
            pawn_handle.clone(),
            (1, i),
        );
    }

    spawn_rook(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        rook_handle.clone(),
        (7, 0),
    );
    spawn_knight(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (7, 1),
    );
    spawn_bishop(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        bishop_handle.clone(),
        (7, 2),
    );
    spawn_queen(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        queen_handle,
        (7, 3),
    );
    spawn_king(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        king_handle,
        king_cross_handle,
        (7, 4),
    );
    spawn_bishop(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        bishop_handle,
        (7, 5),
    );
    spawn_knight(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        knight_1_handle,
        knight_2_handle,
        (7, 6),
    );
    spawn_rook(
        &mut commands,
        dark_material.clone(),
        PieceColor::Dark,
        rook_handle,
        (7, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            dark_material.clone(),
            PieceColor::Dark,
            pawn_handle.clone(),
            (6, i),
        );
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        // Get the direction to move in
        let direction = Vec3::new(piece.x as f32, 0.0, piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    (x, y): (u8, u8),
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(x as f32, 0.0, y as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::King,
            x,
            y,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            parent.spawn_bundle(PbrBundle {
                mesh: mesh_cross,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_knight(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    (x, y): (u8, u8),
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(x as f32, 0.0, y as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Knight,
            x,
            y,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: mesh_1,
                material: material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
            parent.spawn_bundle(PbrBundle {
                mesh: mesh_2,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, 0.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    (x, y): (u8, u8),
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(x as f32, 0.0, y as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Queen,
            x,
            y,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, -0.95));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    (x, y): (u8, u8),
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(x as f32, 0.0, y as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Bishop,
            x,
            y,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0.0, 0.0));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    (x, y): (u8, u8),
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(x as f32, 0.0, y as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Rook,
            x,
            y,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.1, 0.0, 1.8));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    (x, y): (u8, u8),
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(x as f32, 0.0, y as f32)),
            ..Default::default()
        })
        .insert(Piece {
            color: piece_color,
            piece_type: PieceType::Pawn,
            x,
            y,
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh,
                material,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, 2.6));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    Light,
    Dark,
}

impl PieceColor {
    pub fn opponent(&self) -> Self {
        use PieceColor::*;
        match self {
            Light => Dark,
            Dark => Light,
        }
    }
}

impl std::fmt::Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceColor::Light => f.write_str("White"),
            PieceColor::Dark => f.write_str("Black"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub x: u8,
    pub y: u8,
}

impl Piece {
    /// Returns whether the move is valid
    pub fn is_move_valid(&self, new_pos @ (x, y): (u8, u8), pieces: Vec<Piece>) -> bool {
        // TODO en passant, castling
        // If there's a piece of the same color in the same square, it can't move
        let color_of_new = color_of_square(new_pos, &pieces);
        if color_of_new == Some(self.color) {
            return false;
        }

        let pos = (self.x, self.y);
        let x_diff = self.x.abs_diff(x);
        let y_diff = self.y.abs_diff(y);
        let is_path_empty = is_path_empty(pos, new_pos, &pieces);
        match self.piece_type {
            PieceType::King => {
                // Horizontal
                x_diff == 1 && self.y == y
                    // Vertical
                    || y_diff == 1 && self.x == x
                    // Diagonal
                    || x_diff == 1 && y_diff == 1
            }
            PieceType::Queen => {
                is_path_empty
                    && (x_diff == y_diff
                        || (self.x == x && self.y != y)
                        || (self.y == y && self.x != x))
            }
            PieceType::Bishop => is_path_empty && x_diff == y_diff,
            PieceType::Knight => x_diff == 2 && y_diff == 1 || y_diff == 2 && x_diff == 1,
            PieceType::Rook => {
                is_path_empty && (self.y == y && self.x != x || self.x == x && self.y != y)
            }
            PieceType::Pawn => {
                match self.color {
                    PieceColor::Light => {
                        // Has to move forward
                        if x <= self.x {
                            return false;
                        }
                        // Normal move
                        if x_diff == 1 && self.y == y && color_of_new.is_none() {
                            return true;
                        }

                        // Move 2 squares
                        if self.x == 1
                            && x_diff == 2
                            && self.y == y
                            && is_path_empty
                            && color_of_new.is_none()
                        {
                            return true;
                        }

                        // Take piece
                        if x_diff == 1 && y_diff == 1 && color_of_new == Some(PieceColor::Dark) {
                            return true;
                        }
                    }
                    PieceColor::Dark => {
                        // Has to move forward
                        if x >= self.x {
                            return false;
                        }
                        // Normal move
                        if x_diff == 1 && self.y == y && color_of_new.is_none() {
                            return true;
                        }

                        // Move 2 squares
                        if self.x == 6
                            && x_diff == 2
                            && self.y == y
                            && is_path_empty
                            && color_of_new.is_none()
                        {
                            return true;
                        }

                        // Take piece
                        if x_diff == 1 && y_diff == 1 && color_of_new == Some(PieceColor::Light) {
                            return true;
                        }
                    }
                }
                false
            }
        }
    }
}

/// Returns None if the square is empty, retuns Some with the color if not
fn color_of_square((x, y): (u8, u8), pieces: &[Piece]) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == x && piece.y == y {
            return Some(piece.color);
        }
    }
    None
}

fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &[Piece]) -> bool {
    // Same column
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }
    // Same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }
    let x_diff = begin.0.abs_diff(end.0);
    let y_diff = begin.1.abs_diff(end.1);
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                // left bottom - right top
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                // left top - right bottom
                (begin.0 + i as u8, begin.1 - i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                // right bottom - left top
                (begin.0 - i as u8, begin.1 + i as u8)
            } else {
                // right top - left bottom
                (begin.0 - i as u8, begin.1 - i as u8)
            };
            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}
