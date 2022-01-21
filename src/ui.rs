use bevy::prelude::*;

// Component to mark the Text entity
#[derive(Component)]
struct NextMoveText;

/// Initialize UiCamera and text
fn init_next_move_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Next move: White",
                    TextStyle {
                        font,
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                        ..Default::default()
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Left,
                    },
                ),
                ..Default::default()
            }).insert(NextMoveText);
        });
}

/// Update text with the correct turn
fn next_move_text_update(
    turn: Res<crate::board::PlayerTurn>,
    mut query: Query<&mut Text, With<NextMoveText>>,
) {
    if !turn.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        for section in text.sections.iter_mut() {
            println!("  found section");
            section.value = format!("Next move: {}", turn.0);
        }
    }
}

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_next_move_text)
            .add_system(next_move_text_update);
    }
}
