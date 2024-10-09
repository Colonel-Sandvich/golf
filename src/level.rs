use avian2d::prelude::*;
use bevy::{
    math::vec2,
    prelude::*,
    render::primitives::Aabb,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2d},
};

use crate::{
    ball::{Ball, BallResetEvent},
    cam::on_level_resize_zoom,
    course::NextLevelIndex,
    level_data::Levels,
};

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Hash)]
pub enum LevelState {
    #[default]
    Playable,
    InPlay,
    Won,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelState>();

        app.add_systems(Startup, setup);

        app.add_systems(
            Update,
            detect_ball_in_goal.run_if(in_state(LevelState::InPlay)),
        );

        app.add_systems(
            Update,
            tick_level_transition_timer.run_if(in_state(LevelState::Won)),
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
}

pub fn load_level(
    mut commands: Commands,
    levels: Res<Levels>,
    next_level_index: Res<NextLevelIndex>,
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
) {
    if levels.is_empty() {
        return;
    }

    let Ok((level_entity, mut mesh, mut material, mut collider, mut tee)) =
        level_q.get_single_mut()
    else {
        return;
    };

    let Some(next_level) = &levels.0.get(**next_level_index) else {
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
            next_level_state.set(LevelState::Won);
        }
    }
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
        next_level_state.set(LevelState::Playable);
        timer.reset();
    }
}
