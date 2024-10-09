use bevy::{math::vec2, prelude::*, render::camera::CameraUpdateSystem};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup.after(CameraUpdateSystem));

        app.add_systems(Update, follow_camera);
    }
}

const PADDING: f32 = 50.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;

#[derive(Component)]
struct Background;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cam_q: Query<&OrthographicProjection, With<IsDefaultUiCamera>>,
) {
    let beautiful = asset_server.load::<Image>("images/pixel.png");

    let projection = cam_q.single();

    let mut size = projection.area.size();
    size += vec2(PADDING, PADDING * ASPECT_RATIO);

    commands.spawn((
        Background,
        SpriteBundle {
            texture: beautiful.clone(),
            transform: Transform::from_xyz(0.0, 0.0, -1000.0),
            sprite: Sprite {
                custom_size: Some(size),
                ..default()
            },
            ..default()
        },
    ));
}

fn follow_camera(
    cam_q: Query<&Transform, (With<IsDefaultUiCamera>, Changed<Transform>)>,
    mut background_q: Query<&mut Transform, (With<Background>, Without<IsDefaultUiCamera>)>,
) {
    let Ok(cam_transform) = cam_q.get_single() else {
        return;
    };

    let mut background_transform = background_q.single_mut();

    background_transform.translation.x = cam_transform.translation.x;
    background_transform.translation.y = cam_transform.translation.y;
}
