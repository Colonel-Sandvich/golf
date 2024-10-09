use avian2d::prelude::*;
use bevy::{audio::Volume, prelude::*};

use crate::{
    ball::{Ball, BallHitEvent},
    level::LevelState,
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PostUpdate, react_to_ball_hit);
        app.add_systems(OnEnter(LevelState::Win), spawn_firework_sounds);
        app.add_systems(PostUpdate, play_firework_sounds);
        app.add_systems(PostProcessCollisions, play_ball_bounce_sound);
    }
}

#[derive(Resource)]
struct BallHitSound(pub Handle<AudioSource>);

#[derive(Resource)]
struct BallBounceSound(pub Handle<AudioSource>);

#[derive(Component, Clone)]
struct FireworkDelay(pub Timer);

#[derive(Resource)]
struct FireworkSounds(pub Vec<(Handle<AudioSource>, FireworkDelay)>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_hit = asset_server.load::<AudioSource>("sounds/ball_hit.ogg");
    commands.insert_resource(BallHitSound(ball_hit));

    let ball_bounce = asset_server.load::<AudioSource>("sounds/ball_bounce.ogg");
    commands.insert_resource(BallBounceSound(ball_bounce));

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
                mode: bevy::audio::PlaybackMode::Despawn,
                // Swing sound is too loud (-0.2) temp fix
                volume: Volume::new((vel.log(MAX_HIT_VELOCITY) - 0.2).max(0.0)),
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
                    mode: bevy::audio::PlaybackMode::Despawn,
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

const MAX_FORCE_BOUNCE: f32 = 320_000_000.0;

fn play_ball_bounce_sound(
    mut commands: Commands,
    collisions: Res<Collisions>,
    ball_q: Query<(Entity, &Mass), With<Ball>>,
    gravity: Res<Gravity>,
    sound: Res<BallBounceSound>,
    time: Res<Time<Substeps>>,
) {
    let Ok((ball_entity, ball_mass)) = ball_q.get_single() else {
        return;
    };

    for collision in collisions.collisions_with_entity(ball_entity) {
        if collision.is_sensor {
            continue;
        }

        let weight = ball_mass.0 * gravity.0.y;

        let normal_force = collision.total_normal_impulse / time.delta_seconds();

        let net_force = normal_force + weight;

        let volume = (net_force / MAX_FORCE_BOUNCE).sqrt().clamp(0.0, 1.0);

        if volume > 0.05 {
            commands.spawn(AudioBundle {
                source: sound.0.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: Volume::new(volume),
                    ..default()
                },
            });

            return;
        }
    }
}
