use bevy::{
    color::palettes::tailwind::PURPLE_900,
    math::{
        bounding::{Aabb2d, BoundingVolume},
        vec2,
    },
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages},
};
use earcutr::earcut;

use crate::level::BALL_RADIUS;

pub struct LevelDataPlugin;

impl Plugin for LevelDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Levels>();

        app.add_systems(PreStartup, load_levels);
    }
}

#[derive(Resource, Default, Deref)]
pub struct Levels(pub Vec<Level>);

pub struct Level {
    pub points: Vec<Vec2>,
    pub goal_bottom_left: Vec2,
    pub tee: Vec2,
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

fn load_levels(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(Levels(
        vec![level_1(), level_2(), level_3()]
            .into_iter()
            .map(
                |RawLevelData {
                     directions,
                     goal_index,
                     tee,
                 }| {
                    let points = directions_to_points(directions);
                    let goal_bottom_left = points[goal_index];
                    let mesh = meshes.add(convert_level_points_to_mesh(&points));
                    let tee = tee + points[0] + Vec2::Y * BALL_RADIUS;
                    Level {
                        points,
                        goal_bottom_left,
                        tee,
                        mesh,
                        material: materials.add(ColorMaterial::from_color(PURPLE_900)),
                    }
                },
            )
            .collect(),
    ));
}

struct RawLevelData {
    directions: Vec<Vec2>,
    goal_index: usize,
    tee: Vec2,
}

fn level_1() -> RawLevelData {
    RawLevelData {
        directions: vec![
            Vec2::ZERO,
            Vec2::X * 300.0,
            vec2(100.0, 50.0),
            Vec2::X * 200.0,
            Vec2::Y * -75.0,
            Vec2::X * 50.0,
            Vec2::Y * 75.0,
            Vec2::X * 70.0,
            Vec2::Y * -590.0,
            Vec2::X * -720.0,
            Vec2::Y * 540.0,
        ],
        goal_index: 4,
        tee: vec2(50.0, 0.0),
    }
}

fn level_2() -> RawLevelData {
    RawLevelData {
        directions: vec![
            Vec2::ZERO,
            Vec2::X * 350.0,
            vec2(100.0, 150.0),
            Vec2::X * 200.0,
            Vec2::Y * -75.0,
            Vec2::X * 50.0,
            Vec2::Y * 75.0,
            Vec2::X * 70.0,
            Vec2::Y * -690.0,
            Vec2::X * -770.0,
            Vec2::Y * 540.0,
        ],
        goal_index: 4,
        tee: vec2(50.0, 0.0),
    }
}

fn level_3() -> RawLevelData {
    RawLevelData {
        directions: vec![
            Vec2::ZERO,
            Vec2::X * 150.0,
            vec2(50.0, -50.0),
            vec2(75.0, 50.0),
            vec2(75.0, -50.0),
            vec2(50.0, 50.0),
            Vec2::X * 50.0,
            Vec2::Y * -75.0,
            Vec2::X * 50.0,
            Vec2::Y * 75.0,
            Vec2::X * 50.0,
            vec2(20.0, 20.0),
            Vec2::Y * -400.0,
            Vec2::X * -570.0,
            Vec2::Y * 380.0,
        ],
        goal_index: 7,
        tee: vec2(50.0, 0.0),
    }
}

/// Turn a list of vectors of how to go from one coordinate to the next into a list of those coordinates centred at the shape's centre
fn directions_to_points(directions: Vec<Vec2>) -> Vec<Vec2> {
    let points: Vec<_> = directions
        .into_iter()
        .scan(Vec2::ZERO, |running, vec| {
            *running += vec;
            Some(*running)
        })
        .collect();

    let centre = Aabb2d::from_point_cloud(Vec2::ZERO, Rot2::IDENTITY, &points).center();

    points.into_iter().map(|p| p - centre).collect()
}

fn convert_level_points_to_mesh(points: &Vec<Vec2>) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let positions: Vec<[f32; 3]> = points.iter().map(|&p| [p.x, p.y, 0.0]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    let uvs: Vec<[f32; 2]> = points.iter().map(|&p| p.to_array()).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    let flattened_points: Vec<f32> = points.iter().flat_map(|p| p.to_array().to_vec()).collect();

    let indices = earcut(&flattened_points, &Vec::new(), 2)
        .map(|x| x.into_iter().map(|i| i as u32).collect::<Vec<_>>())
        .expect(&format!(
            "Could not make mesh with these invalid points {:?}",
            points
        ));
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
