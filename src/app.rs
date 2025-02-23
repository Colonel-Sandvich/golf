use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.enable_state_scoped_entities::<AppState>();
    }
}

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Hash, Reflect)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}
