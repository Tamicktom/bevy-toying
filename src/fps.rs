//* Libraries imports
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::{
    App, Color, Commands, Component, Entity, GlobalZIndex, Node, Plugin, PositionType, Query, Res,
    Startup, TextColor, TextFont, Update, Val, With, default,
};
use bevy::text::TextSpan;
use bevy::ui::widget::{Text, TextUiWriter};

#[derive(Component)]
struct FpsText;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, setup_fps_text)
            .add_systems(Update, update_fps_text);
    }
}

fn setup_fps_text(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            GlobalZIndex(999),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("FPS: "),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    FpsText,
                ))
                .with_child((
                    TextSpan::default(),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                ));
        });
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity, With<FpsText>>,
    mut writer: TextUiWriter,
) {
    if let Ok(entity) = query.single()
        && let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        && let Some(value) = fps.smoothed()
    {
        *writer.text(entity, 1) = format!("{value:.0}");
    }
}
