use bevy::{audio::Volume, prelude::*};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_music = asset_server.load::<AudioSource>("songs/otherside.ogg");

    commands.spawn((
        AudioPlayer(background_music),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::new(0.2),
            ..default()
        },
    ));

    // TODO: Test when assets are missing
}
