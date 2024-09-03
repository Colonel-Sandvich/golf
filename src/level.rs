use avian2d::{math::*, prelude::*};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    sprite::MaterialMesh2dBundle,
};

use crate::{ball::Ball, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelState>();
        app.insert_resource(Gravity(Vector::NEG_Y * 9.81 * 80.0));
        app.init_resource::<Tee>();
        app.add_systems(PostStartup, setup)
            .add_systems(
                PostProcessCollisions,
                detect_ball_in_goal.run_if(in_state(LevelState::InPlay)),
            )
            .add_systems(OnEnter(LevelState::Failed), || println!("Fail"))
            .add_systems(OnEnter(LevelState::Success), || println!("Win!"));
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
        Vec2::Y * 25.0,
        Vec2::X * 800.0,
        vec2(100.0, 50.0),
        Vec2::X * 200.0,
        Vec2::Y * -75.0,
        Vec2::X * 50.0,
        Vec2::Y * 75.0,
        Vec2::X * 130.0,
        Vec2::Y * -100.0,
        Vec2::X * -1280.0,
    ];
    let mut running = Vec2::ZERO;
    for vec in &mut points {
        let temp = vec.clone();
        *vec += running;
        running += temp;
    }
    // dbg!(&points);

    let color = Color::BLACK.to_linear().to_f32_array();
    let level_mesh = meshes.add(
        Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default())
            .with_inserted_attribute(
                Mesh::ATTRIBUTE_POSITION,
                points
                    .clone()
                    .iter()
                    .map(|p| p.extend(0.0).to_array())
                    .collect::<Vec<[f32; 3]>>(),
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, vec![color; points.len()]),
    );
    let goal_bl = points[5].clone().extend(0.0);

    let collider = Collider::polyline(points, None);
    // let collider = BoxedPolygon::new(points).collider();

    let square_size = 50.0;
    let square = Rectangle::new(30.0 * square_size, square_size);

    let window_bl = vec3(-WINDOW_WIDTH / 2.0, -WINDOW_HEIGHT / 2.0, 0.0);

    commands.spawn((
        Name::new("Floor"),
        ColorMesh2dBundle {
            mesh: level_mesh.into(),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(window_bl + Vec3::Y * 25.0),
            ..default()
        },
        collider,
        RigidBody::Static,
        Friction::new(1.0),
        Restitution::new(0.0),
    ));

    let circle_radius = 7.5;
    let circle = Circle::new(circle_radius);

    tee.0 = vec3(-300.0, -300.0, 0.0);

    commands.spawn((
        Name::new("Ball"),
        Ball,
        MaterialMesh2dBundle {
            mesh: meshes.add(circle).into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(tee.0),
            ..default()
        },
        circle.collider(),
        RigidBody::Dynamic,
        Friction::new(1.0),
        AngularDamping(2.0),
        Restitution::new(0.0),
    ));

    commands.spawn((
        Name::new("Goal"),
        Goal,
        Collider::rectangle(50.0, 50.0),
        Sensor,
        Transform::from_translation(goal_bl + window_bl + vec3(25.0, 50.0, 0.0)),
    ));

    // let triangle = Triangle2d::new(vec2(0.0, 0.0), vec2(25.0, 50.0), vec2(50.0, 0.0));

    // commands.spawn((
    //     Name::new("Ramp"),
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(triangle).into(),
    //         material: materials.add(Color::BLACK),
    //         transform: Transform::from_xyz(0.0, -square_size * 6.5, 0.0),
    //         ..default()
    //     },
    //     triangle.collider(),
    //     RigidBody::Static,
    //     Friction::new(1.0),
    //     Restitution::new(0.6),
    // ));
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
            next_level_state.set(LevelState::Success);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum LevelState {
    #[default]
    Playable,
    InPlay,
    Failed,
    Success,
}
