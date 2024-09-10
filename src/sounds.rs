use bevy::{audio::Volume, prelude::*};

use crate::{ball::BallHitEvent, level::LevelState};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PostUpdate, react_to_ball_hit);
        app.add_systems(OnEnter(LevelState::Win), spawn_firework_sounds);
        app.add_systems(PostUpdate, play_firework_sounds);
    }
}

#[derive(Resource)]
struct BallHitSound(pub Handle<AudioSource>);

#[derive(Component, Clone)]
struct FireworkDelay(pub Timer);

#[derive(Resource)]
struct FireworkSounds(pub Vec<(Handle<AudioSource>, FireworkDelay)>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_hit = asset_server.load::<AudioSource>("sounds/ball_hit.ogg");
    commands.insert_resource(BallHitSound(ball_hit));

    let firework_launch = asset_server.load::<AudioSource>("sounds/firework_launch.ogg");
    let firework_large_blast_far =
        asset_server.load::<AudioSource>("sounds/firework_large_blast_far.ogg");
    let firework_twinkle_far = asset_server.load::<AudioSource>("sounds/firework_twinkle_far.ogg");

    commands.insert_resource(FireworkSounds(vec![
        (
            firework_launch,
            FireworkDelay(Timer::from_seconds(0.0, TimerMode::Once)),
        ),
        (
            firework_large_blast_far,
            FireworkDelay(Timer::from_seconds(0.5, TimerMode::Once)),
        ),
        (
            firework_twinkle_far,
            FireworkDelay(Timer::from_seconds(0.8, TimerMode::Once)),
        ),
    ]));
}

const MAX_HIT_VELOCITY: f32 = 2000.0;

fn react_to_ball_hit(
    mut commands: Commands,
    mut event_reader: EventReader<BallHitEvent>,
    sound: Res<BallHitSound>,
) {
    for event in event_reader.read() {
        let vel = event.speed.clamp(1.0, MAX_HIT_VELOCITY);
        commands.spawn(AudioBundle {
            source: sound.0.clone(),
            settings: PlaybackSettings {
                volume: Volume::new(vel.log(MAX_HIT_VELOCITY)),
                ..default()
            },
        });
    }
}

fn spawn_firework_sounds(mut commands: Commands, sounds: Res<FireworkSounds>) {
    for (sound, delay) in &sounds.0 {
        commands.spawn((
            AudioBundle {
                source: sound.clone(),
                settings: PlaybackSettings {
                    paused: true,
                    mode: bevy::audio::PlaybackMode::Once,
                    ..default()
                },
            },
            delay.clone(),
        ));
    }
}

fn play_firework_sounds(
    mut firework_q: Query<(&mut AudioSink, &mut FireworkDelay)>,
    time: Res<Time>,
) {
    for (playback, mut timer) in firework_q.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            playback.play();
        }
    }
}
