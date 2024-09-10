use avian2d::{math::*, prelude::*};
use bevy::{
    color::palettes::tailwind::PURPLE_900,
    math::{vec2, vec3},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages},
    sprite::MaterialMesh2dBundle,
};
use earcutr::earcut;

use crate::{ball::Ball, WINDOW_WIDTH};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelState>();
        app.insert_resource(Gravity(Vector::NEG_Y * 9.81 * 80.0));
        app.init_resource::<Tee>();
        app.add_systems(PostStartup, setup).add_systems(
            PostProcessCollisions,
            detect_ball_in_goal.run_if(in_state(LevelState::InPlay)),
        );
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct Tee(pub Vec3);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut tee: ResMut<Tee>,
) {
    let mut points = vec![
        Vec2::ZERO,
        Vec2::X * 300.0,
        vec2(100.0, 50.0),
        Vec2::X * 200.0,
        Vec2::Y * -75.0,
        Vec2::X * 50.0,
        Vec2::Y * 75.0,
        Vec2::X * 70.0,
        Vec2::Y * -1025.0,
        Vec2::X * -720.0,
        Vec2::Y * 925.0,
    ];
    let mut running = Vec2::ZERO;
    for vec in &mut points {
        let temp = *vec;
        *vec += running;
        running += temp;
    }

    let flattened_points: Vec<f32> = points.iter().flat_map(|p| vec![p[0], p[1]]).collect();

    // Triangulate the polygon, resulting in indices for a triangle mesh
    let Ok(indices) = earcut(&flattened_points, &Vec::new(), 2) else {
        panic!("AA!!");
    };

    // Create a Bevy mesh
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // Set the positions (converting points into Vec3, where z is 0)
    let positions: Vec<[f32; 3]> = points.iter().map(|&p| [p[0], p[1], 0.0]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    let uvs: Vec<[f32; 2]> = points.iter().map(|&p| [p[0], p[1]]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh.insert_indices(Indices::U32(
        indices.into_iter().map(|i| i as u32).collect(),
    ));

    let goal_bl = points[4].extend(0.0);

    let start = vec3(-WINDOW_WIDTH / 2.0, -200.0, 0.0);

    let collider = Collider::polyline(points, None);

    let floor = commands
        .spawn((
            Name::new("Floor"),
            ColorMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(ColorMaterial::from_color(PURPLE_900)),
                transform: Transform::from_translation(start + Vec3::Y * 25.0),
                ..default()
            },
            collider,
            RigidBody::Static,
            Friction::new(0.4),
            Restitution::new(0.3),
        ))
        .id();

    let ball_radius = 7.5;
    let ball = Circle::new(ball_radius);

    tee.0 = Vec3::Y * ball_radius + Vec3::X * 50.0 + start + Vec3::Y * 25.0;

    commands.spawn((
        Name::new("Ball"),
        Ball,
        MaterialMesh2dBundle {
            mesh: meshes.add(ball).into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(tee.0),
            ..default()
        },
        ball.collider(),
        RigidBody::Dynamic,
        Friction::new(0.4),
        AngularDamping(4.0),
        // LockedAxes::ROTATION_LOCKED,
        Restitution::new(0.4),
        SweptCcd::LINEAR,
    ));

    commands
        .spawn((
            Name::new("Goal"),
            Goal,
            Collider::rectangle(50.0, 50.0),
            Sensor,
            Transform::from_translation(goal_bl + vec3(25.0, 25.0, 0.0)),
        ))
        .set_parent(floor);
}

#[derive(Component)]
pub struct Goal;

fn detect_ball_in_goal(
    ball_q: Query<Entity, With<Ball>>,
    goal_collisions_q: Query<&CollidingEntities, With<Goal>>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    for entities in &goal_collisions_q {
        if entities.contains(&ball_q.single()) {
            println!("GOAL!");
            next_level_state.set(LevelState::Win);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum LevelState {
    #[default]
    Playable,
    InPlay,
    Failed,
    Win,
}
