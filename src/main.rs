use bevy::prelude::*;
use bevy_mod_picking::{DebugCursorPickingPlugin, DefaultPickingPlugins, PickingCameraBundle};

mod board;
mod pieces;
mod ui;

use board::BoardPlugin;
use pieces::PiecesPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(UiPlugin)
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess!".to_string(),
            width: 1600.,
            height: 1600.,
            ..Default::default()
        })
        .add_startup_system(setup)
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
