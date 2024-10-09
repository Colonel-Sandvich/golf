use bevy::{audio::Volume, prelude::*};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pigstep = asset_server.load::<AudioSource>("songs/otherside.ogg");

    commands.spawn(AudioBundle {
        source: pigstep,
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::new(0.2),
            ..default()
        },
    });

    // TODO: Test when assets are missing
}
