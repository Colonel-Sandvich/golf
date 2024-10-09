use std::f32::consts::PI;

use avian2d::prelude::*;
use bevy::{
    color::palettes::{css::WHITE, tailwind::GRAY_600},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    app::AppState,
    ball::{Ball, BallHitEvent},
    level::LevelState,
    mouse::MouseCoords,
    physics::PhysicsState,
};

pub struct SwingPlugin;

impl Plugin for SwingPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SwingState>()
            .init_resource::<StartOfSwing>()
            .init_resource::<Swing>();

        app.add_systems(
            Startup,
            (make_colours, spawn_swing_chain, spawn_swing_start_marker).chain(),
        );

        app.add_systems(
            PreUpdate,
            (start_swing, move_swing_start_marker).chain().run_if(
                in_state(AppState::InGame)
                    .and_then(in_state(LevelState::Playable))
                    .and_then(in_state(SwingState::None))
                    .and_then(in_state(PhysicsState::Running)),
            ),
        )
        .add_systems(
            Update,
            (calculate_swing_power, display_swing, swing)
                .chain()
                .run_if(in_state(AppState::InGame).and_then(in_state(SwingState::WindUp))),
        )
        .add_systems(
            OnEnter(SwingState::WindUp),
            |mut swing_start: Query<&mut Visibility, With<SwingStartMarker>>| {
                *swing_start.single_mut() = Visibility::Visible;
            },
        )
        .add_systems(
            OnExit(SwingState::WindUp),
            |mut swing_display_q: Query<&mut Visibility, With<SwingDisplay>>| {
                for mut nodes in swing_display_q.iter_mut() {
                    *nodes = Visibility::Hidden;
                }
            },
        );
    }
}

#[derive(Resource)]
struct Colours {
    gray: Handle<ColorMaterial>,
    white: Handle<ColorMaterial>,
}

fn make_colours(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(Colours {
        gray: materials.add(ColorMaterial::from_color(GRAY_600)),
        white: materials.add(ColorMaterial::from_color(WHITE)),
    });
}

#[derive(Component)]
struct SwingStartMarker;

fn spawn_swing_start_marker(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let rect = Rectangle::new(20.0, 5.0);
    let rect_mesh = meshes.add(rect);

    let white = materials.add(ColorMaterial::from_color(WHITE));

    let marker = commands
        .spawn((
            SwingStartMarker,
            SwingDisplay,
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.5),
                visibility: Visibility::Hidden,
                ..default()
            },
        ))
        .id();

    commands
        .spawn(ColorMesh2dBundle {
            mesh: rect_mesh.clone().into(),
            material: white.clone(),
            ..default()
        })
        .set_parent(marker);

    commands
        .spawn(ColorMesh2dBundle {
            mesh: rect_mesh.clone().into(),
            material: white.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_z(PI / 2.0)),
            ..default()
        })
        .set_parent(marker);
}

#[derive(Component)]
struct SwingDisplay;

#[derive(Component)]
struct ChainStart;

#[derive(Component)]
struct ChainIndex(pub u8);

const CHAIN_LENGTH: u8 = 7;
const CHAIN_SPACING: u8 = 25;

const WINDUP_DISTANCE_IN_PIXELS: f32 = ((CHAIN_LENGTH + 1) * CHAIN_SPACING) as f32;

fn spawn_swing_chain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colours: Res<Colours>,
) {
    let ball_radius = 5.0;
    let ball = Circle::new(ball_radius);
    let mesh: Mesh2dHandle = meshes.add(ball).into();

    let chain_parent = commands
        .spawn((
            SwingDisplay,
            ChainStart,
            SpatialBundle {
                visibility: Visibility::Hidden,
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
        ))
        .id();

    for i in 1..=CHAIN_LENGTH {
        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: mesh.clone(),
                    material: colours.gray.clone(),
                    transform: Transform::from_xyz((CHAIN_SPACING * i) as f32, 0.0, 0.0),
                    ..default()
                },
                ChainIndex(i),
            ))
            .set_parent(chain_parent);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum SwingState {
    #[default]
    None,
    WindUp,
}

#[derive(Resource, Default)]
struct StartOfSwing(pub Vec2);

fn start_swing(
    mouse_click: Res<ButtonInput<MouseButton>>,
    mouse_coords: Res<MouseCoords>,
    mut start_pos: ResMut<StartOfSwing>,
    mut next_state: ResMut<NextState<SwingState>>,
) {
    if !mouse_click.just_pressed(MouseButton::Left) {
        return;
    }

    start_pos.0 = mouse_coords.0;

    next_state.set(SwingState::WindUp);
}

fn move_swing_start_marker(
    mut swing_marker_q: Query<&mut Transform, With<SwingStartMarker>>,
    start_of_swing: Res<StartOfSwing>,
) {
    let mut swing_marker = swing_marker_q.single_mut();
    swing_marker.translation.x = start_of_swing.0.x;
    swing_marker.translation.y = start_of_swing.0.y;
}

#[derive(Resource, Default)]
pub struct Swing {
    pub power: u8,
    pub angle: f32,
}

fn calculate_swing_power(
    mouse_coords: Res<MouseCoords>,
    start_of_swing: Res<StartOfSwing>,
    mut swing: ResMut<Swing>,
) {
    let dir = (start_of_swing.0 - mouse_coords.0).clamp_length_max(WINDUP_DISTANCE_IN_PIXELS);

    let power_level =
        (dir.length() / WINDUP_DISTANCE_IN_PIXELS * CHAIN_LENGTH as f32).round() as u8;

    swing.power = power_level;
    swing.angle = dir.to_angle();
}

fn display_swing(
    swing: Res<Swing>,
    ball_q: Query<&Position, With<Ball>>,
    mut chain_parent_q: Query<(&mut Transform, &mut Visibility), With<ChainStart>>,
    mut chain_q: Query<(&mut Handle<ColorMaterial>, &ChainIndex)>,
    colours: Res<Colours>,
) {
    let Ok(ball_pos) = ball_q.get_single() else {
        return;
    };

    let (mut transform, mut visiblity) = chain_parent_q.single_mut();

    if swing.power < 1 {
        *visiblity = Visibility::Hidden;
        return;
    }

    *visiblity = Visibility::Visible;

    transform.translation.x = ball_pos.x;
    transform.translation.y = ball_pos.y;
    transform.rotation = Quat::from_rotation_z(swing.angle);

    for (mut color, index) in chain_q.iter_mut() {
        if index.0 <= swing.power {
            *color = colours.white.clone();
        } else {
            *color = colours.gray.clone();
        }
    }
}

const LAUNCH_FACTOR: f32 = 170.0;

fn swing(
    mouse_click: Res<ButtonInput<MouseButton>>,
    swing: Res<Swing>,
    mut ball_q: Query<&mut LinearVelocity, With<Ball>>,
    mut event_writer: EventWriter<BallHitEvent>,
    mut next_state: ResMut<NextState<SwingState>>,
) {
    if !mouse_click.just_released(MouseButton::Left) {
        return;
    }

    next_state.set(SwingState::None);

    if swing.power == 0 {
        return;
    }

    let mut ball_vel = ball_q.single_mut();

    ball_vel.0 = Vec2::from_angle(swing.angle) * swing.power as f32 * LAUNCH_FACTOR;
    // ball_vel.0 = vec2(1200.0, 0.0);

    event_writer.send(BallHitEvent {
        speed: ball_vel.length(),
    });
}
