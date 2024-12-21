use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;

use crate::level::load_level;
use crate::{app::AppState, level::LevelState, level_data::Levels};

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Hash, Reflect)]
pub enum CourseState {
    #[default]
    Playing,
    LoadNextLevel,
    Won,
    Failed,
}

pub struct CoursePlugin;

impl Plugin for CoursePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CourseState>();

        app.init_resource::<NextLevelIndex>();

        app.add_systems(OnExit(LevelState::Won), advance_level_or_win_course);
        app.add_systems(
            OnEnter(CourseState::LoadNextLevel),
            (
                load_level,
                |mut next_course_state: ResMut<NextState<CourseState>>| {
                    next_course_state.set(CourseState::Playing);
                },
            )
                .chain(),
        );

        app.add_systems(
            OnEnter(CourseState::Playing),
            |mut next_level_state: ResMut<NextState<LevelState>>| {
                next_level_state.set(LevelState::Playable);
            },
        );

        app.add_systems(
            OnEnter(AppState::InGame),
            (|mut next_course_state: ResMut<NextState<CourseState>>| {
                next_course_state.set(CourseState::Playing);
            },)
                .chain(),
        );

        app.add_systems(
            OnEnter(AppState::Menu),
            (
                |mut next_level_index: ResMut<NextLevelIndex>| {
                    **next_level_index = 0;
                },
                load_level,
            )
                .chain(),
        );

        // Show level behind main menu
        app.add_systems(PostStartup, load_level);

        app.add_systems(OnEnter(CourseState::Won), display_course_over_screen)
            .add_systems(OnEnter(CourseState::Failed), display_course_over_screen)
            .add_systems(
                Update,
                goto_menu.run_if(in_state(CourseState::Won).or(in_state(CourseState::Failed))),
            );
    }
}

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct NextLevelIndex(usize);

fn advance_level_or_win_course(
    levels: Res<Levels>,
    mut next_level_index: ResMut<NextLevelIndex>,
    mut next_course_state: ResMut<NextState<CourseState>>,
) {
    if **next_level_index + 1 >= levels.0.len() {
        next_course_state.set(CourseState::Won);
        return;
    }

    **next_level_index += 1;

    next_course_state.set(CourseState::LoadNextLevel);
}

#[derive(Component)]
struct CourseOverScreen;

fn display_course_over_screen(mut commands: Commands, course_state: Res<State<CourseState>>) {
    let text = match course_state.get() {
        CourseState::Won => "Le Epic Win!",
        CourseState::Failed => "GAME OVER",
        _ => return,
    };

    commands
        .spawn((
            CourseOverScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(BLACK.with_alpha(0.75).into()),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(text),
                TextFont::from_font_size(120.0),
                TextColor::WHITE,
            ));
        });
}

fn goto_menu(
    input: Res<ButtonInput<MouseButton>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    course_over_q: Query<Entity, With<CourseOverScreen>>,
) {
    if input.just_released(MouseButton::Left) {
        next_app_state.set(AppState::Menu);
        for node in course_over_q.iter() {
            commands.entity(node).despawn_recursive();
        }
    }
}
