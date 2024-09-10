use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    level::{LevelState, Tee},
    state::AppState,
};

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BallOOBEvent>()
            .add_event::<BallStoppedEvent>()
            .add_event::<BallHitEvent>();
        app.add_systems(
            Update,
            (oob_check, reset_ball)
                .chain()
                .run_if(in_state(LevelState::InPlay)),
        );
        app.add_systems(
            Update,
            launch_ball
                .run_if(in_state(LevelState::Playable).and_then(in_state(AppState::Running))),
        );
        app.add_systems(
            PostUpdate,
            stop_ball.run_if(not(in_state(LevelState::Playable))),
        );
    }
}

#[derive(Event)]
pub struct BallOOBEvent;

fn oob_check(
    ball_q: Query<&Transform, With<Ball>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut events: EventWriter<BallOOBEvent>,
) {
    let ball = ball_q.single();

    let Vec2 { x, y } = ball.translation.xy();

    let window = window_q.single();

    if x < -window.width() / 2.0 || x > window.width() / 2.0 || y < -window.height() / 2.0 {
        events.send(BallOOBEvent);
    }
}

fn reset_ball(
    events: EventReader<BallOOBEvent>,
    mut ball_q: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity), With<Ball>>,
    tee: Res<Tee>,
) {
    if events.is_empty() {
        return;
    }

    let (mut ball_pos, mut ball_vel, mut ball_roll) = ball_q.single_mut();

    ball_roll.0 = 0.0;
    ball_vel.0 = Vec2::ZERO;
    ball_pos.translation = tee.0;
}

#[derive(Event, Debug)]
pub struct BallHitEvent {
    pub speed: f32,
}

const LAUNCH_FACTOR: f32 = 2.0;

fn launch_ball(
    mouse_click: Res<ButtonInput<MouseButton>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut ball_q: Query<(&Transform, &mut LinearVelocity), With<Ball>>,
    mut event_writer: EventWriter<BallHitEvent>,
) {
    if !mouse_click.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = camera_q.single();

    let Some(cursor_position) = window_q.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };

    let (ball_pos, mut ball_vel) = ball_q.single_mut();

    let dir = world_position - ball_pos.translation.xy();

    ball_vel.0 = dir * LAUNCH_FACTOR;

    event_writer.send(BallHitEvent {
        speed: ball_vel.length(),
    });
}

#[derive(Event)]
pub struct BallStoppedEvent;

pub const ANGULAR_VEL_THRESHOLD: f32 = 3.0;
pub const VEL_THRESHOLD: f32 = 3.0;

fn stop_ball(
    mut ball_q: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Ball>>,
    mut event_writer: EventWriter<BallStoppedEvent>,
    level_state: Res<State<LevelState>>,
) {
    let (mut ball_vel, mut ball_roll) = ball_q.single_mut();

    if ball_roll.0.abs() < ANGULAR_VEL_THRESHOLD && ball_vel.0.length() < VEL_THRESHOLD {
        ball_roll.0 = 0.0;
        ball_vel.0 = Vec2::ZERO;

        if *level_state == LevelState::InPlay {
            event_writer.send(BallStoppedEvent);
        }
    }
}
