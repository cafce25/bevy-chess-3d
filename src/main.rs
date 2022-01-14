use bevy::prelude::*;

mod pieces;
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(create_board)
        .add_startup_system(create_pieces)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
    });
    // Light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add meshes and materials
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));
    let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());

    // Spawn 64 squares {
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: if (i + j) % 2 == 0 {
                    black_material.clone()
                } else {
                    white_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                ..Default::default()
            });
        }
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
    let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());

    pieces::spawn_rook(
        &mut commands,
        white_material.clone(),
        rook_handle.clone(),
        Vec3::new(0.0, 0.0, 0.0),
    );
    pieces::spawn_knight(
        &mut commands,
        white_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(0.0, 0.0, 1.0),
    );
    pieces::spawn_bishop(
        &mut commands,
        white_material.clone(),
        bishop_handle.clone(),
        Vec3::new(0.0, 0.0, 2.0),
    );
    pieces::spawn_queen(
        &mut commands,
        white_material.clone(),
        queen_handle.clone(),
        Vec3::new(0.0, 0.0, 3.0),
    );
    pieces::spawn_king(
        &mut commands,
        white_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        Vec3::new(0.0, 0.0, 4.0),
    );
    pieces::spawn_bishop(
        &mut commands,
        white_material.clone(),
        bishop_handle.clone(),
        Vec3::new(0.0, 0.0, 5.0),
    );
    pieces::spawn_knight(
        &mut commands,
        white_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(0.0, 0.0, 6.0),
    );
    pieces::spawn_rook(
        &mut commands,
        white_material.clone(),
        rook_handle.clone(),
        Vec3::new(0.0, 0.0, 7.0),
    );

    for i in 0..8 {
        pieces::spawn_pawn(
            &mut commands,
            white_material.clone(),
            pawn_handle.clone(),
            Vec3::new(1.0, 0.0, i as f32),
        );
    }

    pieces::spawn_rook(
        &mut commands,
        black_material.clone(),
        rook_handle.clone(),
        Vec3::new(7.0, 0.0, 0.0),
    );
    pieces::spawn_knight(
        &mut commands,
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7.0, 0.0, 1.0),
    );
    pieces::spawn_bishop(
        &mut commands,
        black_material.clone(),
        bishop_handle.clone(),
        Vec3::new(7.0, 0.0, 2.0),
    );
    pieces::spawn_queen(
        &mut commands,
        black_material.clone(),
        queen_handle.clone(),
        Vec3::new(7.0, 0.0, 3.0),
    );
    pieces::spawn_king(
        &mut commands,
        black_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        Vec3::new(7.0, 0.0, 4.0),
    );
    pieces::spawn_bishop(
        &mut commands,
        black_material.clone(),
        bishop_handle.clone(),
        Vec3::new(7.0, 0.0, 5.0),
    );
    pieces::spawn_knight(
        &mut commands,
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7.0, 0.0, 6.0),
    );
    pieces::spawn_rook(
        &mut commands,
        black_material.clone(),
        rook_handle.clone(),
        Vec3::new(7.0, 0.0, 7.0),
    );

    for i in 0..8 {
        pieces::spawn_pawn(
            &mut commands,
            black_material.clone(),
            pawn_handle.clone(),
            Vec3::new(6.0, 0.0, i as f32),
        );
    }

}
