use bevy::prelude::*;

use crate::app::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_menu);

        app.add_systems(Update, menu.run_if(in_state(AppState::Menu)));
    }
}

#[derive(Component)]
struct MainMenu;

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenu,
            StateScoped(AppState::Menu),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PLAY"),
                TextFont::from_font_size(64.0),
                TextColor::WHITE,
            ));
        });
}

fn menu(input: Res<ButtonInput<MouseButton>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_released(MouseButton::Left) {
        next_state.set(AppState::InGame);
    }
}
