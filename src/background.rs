use bevy::{math::vec2, prelude::*, render::camera::CameraUpdateSystem};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup.after(CameraUpdateSystem));
    }
}

const PADDING: f32 = 50.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;

#[derive(Component)]
struct Background;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cam_q: Query<(Entity, &OrthographicProjection), With<IsDefaultUiCamera>>,
) {
    let beautiful = asset_server.load::<Image>("images/pixel.png");

    let (cam_entity, projection) = cam_q.single();

    let mut size = projection.area.size();
    size += vec2(PADDING, PADDING * ASPECT_RATIO);

    commands.entity(cam_entity).with_child((
        Background,
        Sprite {
            image: beautiful.clone(),
            custom_size: Some(size),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1000.0),
    ));
}
