use bevy::{math::vec2, prelude::*};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let beautiful = asset_server.load::<Image>("images/pixel2.png");

    commands.spawn(SpriteBundle {
        texture: beautiful,
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        sprite: Sprite {
            custom_size: Some(vec2(WINDOW_WIDTH, WINDOW_HEIGHT)),
            ..default()
        },
        ..default()
    });
}
