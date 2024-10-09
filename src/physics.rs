use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .init_state::<PhysicsState>()
            .insert_resource(SleepingThreshold {
                angular: 3.0,
                linear: 3.0,
            })
            .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 80.0))
            .insert_resource(Time::<Fixed>::from_hz(640.0))
            .add_systems(
                OnEnter(PhysicsState::Paused),
                |mut time: ResMut<Time<Physics>>| time.pause(),
            )
            .add_systems(
                OnExit(PhysicsState::Paused),
                |mut time: ResMut<Time<Physics>>| time.unpause(),
            )
            .add_systems(Update, pause_button);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum PhysicsState {
    #[default]
    Running,
    Paused,
}

fn pause_button(
    current_state: ResMut<State<PhysicsState>>,
    mut next_state: ResMut<NextState<PhysicsState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        let new_state = match current_state.get() {
            PhysicsState::Paused => PhysicsState::Running,
            PhysicsState::Running => PhysicsState::Paused,
        };
        next_state.set(new_state);
    }
}
