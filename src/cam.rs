use bevy::{prelude::*, render::primitives::Aabb, window::PrimaryWindow};

use crate::level::Floor;

pub struct CamPlugin;

impl Plugin for CamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LevelBounds>();

        app.add_systems(Startup, spawn_cam)
            .add_systems(Update, update_projection_area);
    }
}

fn spawn_cam(mut commands: Commands) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct LevelBounds(pub Rect);

fn update_projection_area(
    mut level_bounds: ResMut<LevelBounds>,
    camera_q: Query<
        &OrthographicProjection,
        (Changed<OrthographicProjection>, With<IsDefaultUiCamera>),
    >,
) {
    let Ok(projection) = camera_q.get_single() else {
        return;
    };
    level_bounds.0 = projection.area;
}

pub fn on_level_resize_zoom(
    trigger: Trigger<OnAdd, Aabb>,
    mut level_q: Query<&Aabb, With<Floor>>,
    mut camera_q: Query<(&mut OrthographicProjection, &mut Transform)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let height = window_q.single().size().y;

    let level_aabb = level_q.get_mut(trigger.entity()).unwrap();

    let (projection, transform) = &mut camera_q.single_mut();

    let level_width = level_aabb.half_extents.x * 2.0;
    let level_height = level_aabb.half_extents.y * 2.0;

    let factor = level_width / projection.area.width();

    projection.scale *= factor;

    let translation = 0.5 * (height * projection.scale - level_height).abs();

    transform.translation = Vec3::Y * translation;
}
