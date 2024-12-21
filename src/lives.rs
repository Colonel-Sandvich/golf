use bevy::{color::palettes::css::TOMATO, prelude::*};

use crate::{
    app::AppState,
    ball::{BallHitEvent, BallStoppedEvent},
    course::CourseState,
    level::LevelState,
};

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives(2));
        app.insert_resource(LivesLeft(2));

        app.add_systems(Startup, setup);

        app.add_systems(Update, update_lives_left_text);
        app.add_systems(
            Update,
            toggle_lives_left_visibility.run_if(state_changed::<AppState>),
        );

        app.add_systems(PostUpdate, react_to_ball_hit);
        app.add_systems(
            Last,
            react_to_ball_stopped.run_if(in_state(LevelState::InPlay)),
        );

        app.add_systems(OnExit(LevelState::Won), reset_lives_left)
            .add_systems(OnEnter(CourseState::Playing), reset_lives_left);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Lives(u32);

#[derive(Resource, Deref, DerefMut)]
pub struct LivesLeft(u32);

#[derive(Component)]
struct LivesText;

fn setup(mut commands: Commands) {
    commands
        .spawn((Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            padding: UiRect::top(Val::Px(20.0)),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                LivesText,
                Text::new("Lives: "),
                TextFont::from_font_size(20.0),
                TextColor::from(TOMATO),
            ));
        });
}

fn toggle_lives_left_visibility(
    mut query: Query<&mut Visibility, With<LivesText>>,
    course_state: Res<State<AppState>>,
) {
    let Ok(mut visibility) = query.get_single_mut() else {
        return;
    };

    *visibility = if *course_state.get() == AppState::InGame {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}

fn update_lives_left_text(
    lives_left: Res<LivesLeft>,
    mut lives_text_q: Query<&mut Text, With<LivesText>>,
) {
    if !lives_left.is_changed() {
        return;
    }

    for mut text in &mut lives_text_q {
        text.0 = format!("Lives: {}", **lives_left);
    }
}

fn react_to_ball_hit(
    mut lives: ResMut<LivesLeft>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut event_reader: EventReader<BallHitEvent>,
) {
    for _ in event_reader.read() {
        **lives -= 1;

        next_level_state.set(LevelState::InPlay);
    }
}

fn react_to_ball_stopped(
    lives: Res<LivesLeft>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut next_course_state: ResMut<NextState<CourseState>>,
    mut event_reader: EventReader<BallStoppedEvent>,
) {
    for _ in event_reader.read() {
        if **lives == 0 {
            next_course_state.set(CourseState::Failed);
            return;
        }

        next_level_state.set(LevelState::Playable);
    }
}

fn reset_lives_left(mut lives_left: ResMut<LivesLeft>, lives: Res<Lives>) {
    **lives_left = **lives;
}
