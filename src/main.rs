mod background;
mod ball;
mod cam;
mod level;
mod light;
mod lives;
mod music;
mod sounds;
mod state;

use avian2d::{debug_render::PhysicsDebugPlugin, PhysicsPlugins};
use background::BackgroundPlugin;
use ball::BallPlugin;
use bevy::{prelude::*, window::WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use cam::CamPlugin;
use level::LevelPlugin;
use light::LightPlugin;
use lives::LivesPlugin;
use music::MusicPlugin;
use sounds::SoundPlugin;
use state::StatePlugin;

pub const WINDOW_WIDTH: f32 = 720.0;
pub const WINDOW_HEIGHT: f32 = 1280.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                title: "Golf".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(EditorPlugin::default())
        // .insert_resource(ClearColor(Srgba::hex("74b3ff").unwrap().into()))
        .add_plugins(CamPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(PhysicsPlugins::default().with_length_unit(20.0))
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(StatePlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(LivesPlugin)
        .add_plugins(MusicPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(SoundPlugin)
        .run();
}
