use avian2d::prelude::*;
use bevy::{
    color::palettes::css::{TOMATO, WHITE},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    sprite::{Wireframe2dConfig, Wireframe2dPlugin},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{ball::BallResetEvent, physics::PhysicsState};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if !cfg!(debug_assertions) {
            return;
        }

        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(Wireframe2dPlugin)
            .insert_resource(Wireframe2dConfig {
                global: false,
                default_color: WHITE.into(),
            });

        app.add_plugins(PhysicsDebugPlugin::default())
            .insert_gizmo_config(
                PhysicsGizmos {
                    axis_lengths: Some(Vec2::splat(0.5 / 18.0)),
                    ..default()
                },
                GizmoConfig::default(),
            );

        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, update_fps_text);

        app.add_systems(Update, reset_button)
            .add_systems(Update, change_timestep)
            .add_systems(Update, step_button.run_if(in_state(PhysicsState::Paused)));
    }
}

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: TOMATO.into(),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FpsText,
    ));
}

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

fn step_button(mut time: ResMut<Time<Physics>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Enter) {
        let duration = time.delta();
        time.advance_by(duration);
    }
}

fn reset_button(mut events: EventWriter<BallResetEvent>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        events.send(BallResetEvent);
    }
}

fn change_timestep(mut time: ResMut<Time<Fixed>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyT) {
        let hz = 1.0 / time.timestep().as_secs_f64();

        time.set_timestep_hz(if hz == 64.0 { 640.0 } else { 64.0 });
    }
}
