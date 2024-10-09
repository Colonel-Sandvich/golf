use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    cam::LevelBounds,
    level::{Floor, LevelState, Tee},
};

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BallResetEvent>()
            .add_event::<BallStoppedEvent>()
            .add_event::<BallHitEvent>();
        app.add_systems(
            Update,
            (oob_check.run_if(in_state(LevelState::InPlay)), reset_ball).chain(),
        );
        app.add_systems(
            PostUpdate,
            watch_for_stopped_ball.run_if(not(in_state(LevelState::Playable))),
        );
    }
}

#[derive(Event)]
pub struct BallResetEvent;

fn oob_check(
    ball_q: Query<&Position, With<Ball>>,
    level_bounds: Res<LevelBounds>,
    mut events: EventWriter<BallResetEvent>,
) {
    let ball = ball_q.single();

    let is_ball_outside_width = ball.x < level_bounds.min.x || ball.x > level_bounds.max.x;
    let is_ball_below_floor = ball.y < level_bounds.min.y;

    if is_ball_outside_width || is_ball_below_floor {
        events.send(BallResetEvent);
    }
}

fn reset_ball(
    events: EventReader<BallResetEvent>,
    mut ball_q: Query<
        (
            &mut Position,
            &mut Rotation,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut Restitution,
        ),
        With<Ball>,
    >,
    current_level_q: Query<(&Position, &Tee), (With<Floor>, Without<Ball>)>,
) {
    if events.is_empty() {
        return;
    }

    let Ok((mut pos, mut rot, mut vel, mut roll, mut restitution)) = ball_q.get_single_mut() else {
        return;
    };

    let (level_pos, level_tee) = current_level_q.single();

    pos.0 = level_pos.0 + level_tee.0;
    *rot = Rotation::IDENTITY;
    vel.0 = Vec2::ZERO;
    roll.0 = 0.0;
    *restitution = Restitution::new(0.4).with_combine_rule(CoefficientCombine::Average);
}

#[derive(Event, Debug)]
pub struct BallHitEvent {
    pub speed: f32,
}

#[derive(Event)]
pub struct BallStoppedEvent;

fn watch_for_stopped_ball(
    ball_q: Query<(), (With<Ball>, Added<Sleeping>)>,
    mut event_writer: EventWriter<BallStoppedEvent>,
) {
    for _ in ball_q.iter() {
        event_writer.send(BallStoppedEvent);
    }
}
