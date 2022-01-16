use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

pub fn create_board(
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
            commands
                .spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: if (i + j) % 2 == 0 {
                        black_material.clone()
                    } else {
                        white_material.clone()
                    },
                    transform: Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default());
        }
    }
}
