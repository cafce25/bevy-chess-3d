use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

pub fn create_board(
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
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub fn is_light(&self) -> bool {
        (self.x + self.y) % 2 == 1
    }
}
