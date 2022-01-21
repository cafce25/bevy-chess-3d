use bevy::prelude::*;
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DefaultPickingPlugins, Hover, PickingCameraBundle, Selection,
};
use board::Square;

mod board;
mod pieces;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_startup_system(board::create_board)
        .add_startup_system(pieces::create_pieces)
        .add_system(color_squares)
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
