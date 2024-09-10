use bevy::{
    color::palettes::css::{BLACK, TOMATO, WHITE},
    prelude::*,
};

use crate::{
    ball::{BallHitEvent, BallStoppedEvent},
    level::LevelState,
};

pub struct LivesPlugin;

impl Plugin for LivesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lives(2));
        app.insert_resource(LivesLeft(2));
        app.add_systems(PostUpdate, react_to_ball_hit);
        app.add_systems(
            Last,
            react_to_ball_stopped.run_if(in_state(LevelState::InPlay)),
        );
        app.add_systems(Startup, setup);
        app.add_systems(OnEnter(LevelState::Failed), fail)
            .add_systems(
                OnExit(LevelState::Failed),
                |mut commands: Commands, game_over_screen: Query<Entity, With<GameOverScreen>>| {
                    commands
                        .entity(game_over_screen.single())
                        .despawn_recursive();
                },
            )
            .add_systems(OnEnter(LevelState::Win), || println!("Win!"));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Lives(u32);

#[derive(Resource, Deref, DerefMut)]
pub struct LivesLeft(u32);

fn react_to_ball_hit(
    mut lives: ResMut<LivesLeft>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut event_reader: EventReader<BallHitEvent>,
    mut query: Query<&mut Text, With<LivesText>>,
) {
    for _ in event_reader.read() {
        **lives -= 1;

        next_level_state.set(LevelState::InPlay);

        for mut text in &mut query {
            text.sections[0].value = format!("Lives: {}", **lives);
        }
    }
}

fn react_to_ball_stopped(
    lives: Res<LivesLeft>,
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut event_reader: EventReader<BallStoppedEvent>,
) {
    for _ in event_reader.read() {
        if **lives == 0 {
            next_level_state.set(LevelState::Failed);
            return;
        }

        next_level_state.set(LevelState::Playable);
    }
}

#[derive(Component)]
struct LivesText;

fn setup(mut commands: Commands, lives: Res<LivesLeft>) {
    commands.spawn((
        TextBundle::from_section(
            format!("Lives: {}", **lives),
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: TOMATO.into(),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(5.0),
            ..default()
        }),
        LivesText,
    ));
}

#[derive(Component)]
struct GameOverScreen;

fn fail(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(BLACK.with_alpha(0.75).into()),
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "GAME OVER",
                TextStyle {
                    font: default(),
                    font_size: 120.0,
                    color: WHITE.into(),
                },
            ));
        });
}
