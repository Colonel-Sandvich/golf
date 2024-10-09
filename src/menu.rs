use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::app::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(OnExit(AppState::Menu), cleanup_menu);

        app.add_systems(Update, menu.run_if(in_state(AppState::Menu)));
    }
}

#[derive(Component)]
struct Menu;

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            Menu,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PLAY",
                TextStyle {
                    font: default(),
                    font_size: 64.0,
                    color: WHITE.into(),
                },
            ));
        });
}

fn cleanup_menu(mut commands: Commands, menu_q: Query<Entity, With<Menu>>) {
    for menu in menu_q.iter() {
        commands.entity(menu).despawn_recursive();
    }
}

fn menu(input: Res<ButtonInput<MouseButton>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_released(MouseButton::Left) {
        next_state.set(AppState::InGame);
    }
}
