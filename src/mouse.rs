use bevy::{prelude::*, window::PrimaryWindow};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseCoords(Vec2::ZERO));
        app.add_systems(First, watch_mouse);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MouseCoords(pub Vec2);

fn watch_mouse(
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut mouse_coords: ResMut<MouseCoords>,
) {
    let (camera, camera_transform) = camera_q.single();

    let Some(cursor_position) = window_q.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };

    **mouse_coords = world_position;
}
