mod app;
mod background;
mod ball;
mod cam;
mod course;
mod debug;
mod level;
mod level_data;
mod lives;
mod menu;
mod mouse;
mod music;
mod physics;
mod sounds;
mod swing;

use app::AppPlugin;
use background::BackgroundPlugin;
use ball::BallPlugin;
use bevy::prelude::*;
use cam::CamPlugin;
use course::CoursePlugin;
use debug::DebugPlugin;
use level::LevelPlugin;
use level_data::LevelDataPlugin;
use lives::LivesPlugin;
use menu::MenuPlugin;
use mouse::MousePlugin;
use music::MusicPlugin;
use physics::PhysicsPlugin;
use sounds::SoundPlugin;
use swing::SwingPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (720.0, 1280.0).into(),
                        decorations: false,
                        title: "Golf".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(CamPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(AppPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(LevelDataPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(CoursePlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(LivesPlugin)
        .add_plugins(MusicPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(SoundPlugin)
        .add_plugins(MousePlugin)
        .add_plugins(SwingPlugin)
        .run();
}
