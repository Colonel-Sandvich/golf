use avian2d::prelude::*;
use bevy::{
    math::vec2,
    prelude::*,
    render::primitives::Aabb,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2d},
};

use crate::{
    app::AppState,
    ball::{Ball, BallResetEvent},
    cam::on_level_resize_zoom,
    level_data::Levels,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum LevelState {
    #[default]
    Playable,
    InPlay,
    Failed,
    Win,
    LoadNext,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelState>();

        app.init_resource::<NextLevelIndex>();

        app.add_systems(Startup, setup);

        app.add_systems(
            Update,
            detect_ball_in_goal.run_if(in_state(LevelState::InPlay)),
        )
        .add_systems(OnEnter(LevelState::Win), reduce_ball_bounciness);

        app.add_systems(
            Update,
            tick_level_transition_timer.run_if(in_state(LevelState::Win)),
        );

        // Handles being able to show the first level before player clicks "PLAY"
        app.add_systems(
            OnEnter(AppState::InGame),
            advance_level.run_if(in_state(LevelState::LoadNext)),
        );

        app.add_systems(OnEnter(LevelState::LoadNext), advance_level);

        app.add_systems(
            OnEnter(AppState::Menu),
            |mut next_level_index: ResMut<NextLevelIndex>| {
                **next_level_index = 0;
            },
        );
    }
}

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct Goal;

#[derive(Component, Default, Deref, DerefMut)]
pub struct Tee(pub Vec2);

pub const BALL_RADIUS: f32 = 7.5;

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    let floor = commands
        .spawn((
            Name::new("Floor"),
            Floor,
            ColorMesh2dBundle::default(),
            Collider::default(),
            RigidBody::Static,
            Friction::new(0.4),
            Restitution::new(0.4),
            Tee::default(),
            Wireframe2d,
        ))
        .observe(on_level_resize_zoom)
        .id();

    let ball = Circle::new(BALL_RADIUS);

    commands.spawn((
        Name::new("Ball"),
        Ball,
        MaterialMesh2dBundle {
            mesh: meshes.add(ball).into(),
            material: materials.add(Color::WHITE),
            ..default()
        },
        Position::default(),
        ball.collider(),
        RigidBody::Dynamic,
        Friction::new(0.4),
        AngularDamping(8.0),
        Restitution::new(0.4),
        SweptCcd::NON_LINEAR,
    ));

    commands
        .spawn((
            Name::new("Goal"),
            Goal,
            Collider::rectangle(50.0, 50.0),
            Sensor,
            TransformBundle::default(),
        ))
        .set_parent(floor);

    next_level_state.set(LevelState::LoadNext);
}

#[derive(Resource, Default, Deref, DerefMut, Debug)]
struct NextLevelIndex(usize);

fn advance_level(
    mut commands: Commands,
    levels: Res<Levels>,
    mut next_level_index: ResMut<NextLevelIndex>,
    mut level_q: Query<
        (
            Entity,
            &mut Mesh2dHandle,
            &mut Handle<ColorMaterial>,
            &mut Collider,
            &mut Tee,
        ),
        With<Floor>,
    >,
    mut goal_q: Query<&mut Transform, With<Goal>>,
    mut reset_ball_events: EventWriter<BallResetEvent>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    let (level_entity, mut mesh, mut material, mut collider, mut tee) = level_q.single_mut();

    let Some(next_level) = &levels.0.get(**next_level_index) else {
        next_app_state.set(AppState::Menu);
        return;
    };

    mesh.0 = next_level.mesh.clone();

    *material = next_level.material.clone();

    *collider = Collider::polyline(next_level.points.clone(), None);

    tee.0 = next_level.tee;

    reset_ball_events.send(BallResetEvent);

    let mut goal_transform = goal_q.single_mut();

    goal_transform.translation = (next_level.goal_bottom_left + vec2(25.0, 25.0)).extend(0.0);

    // Recompute Aabb since we changed the mesh
    commands.entity(level_entity).remove::<Aabb>();

    **next_level_index += 1;

    next_level_state.set(LevelState::Playable);
}

fn detect_ball_in_goal(
    ball_q: Query<Entity, With<Ball>>,
    goal_collisions_q: Query<&CollidingEntities, With<Goal>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    if goal_collisions_q.is_empty() {
        return;
    }

    let ball_entity = ball_q.single();

    for entities in &goal_collisions_q {
        if entities.contains(&ball_entity) {
            next_level_state.set(LevelState::Win);
        }
    }
}

/// Ensure ball doesn't bounce out of hole
fn reduce_ball_bounciness(mut ball_q: Query<&mut Restitution, With<Ball>>) {
    let mut restitution = ball_q.single_mut();

    *restitution = Restitution::new(0.05).with_combine_rule(CoefficientCombine::Min);
}

#[derive(Deref, DerefMut)]
struct LevelTransitionTimer(pub Timer);

impl Default for LevelTransitionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

fn tick_level_transition_timer(
    mut timer: Local<LevelTransitionTimer>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    time: Res<Time>,
) {
    timer.tick(time.delta());

    if timer.finished() {
        next_level_state.set(LevelState::LoadNext);
        timer.reset();
    }
}
