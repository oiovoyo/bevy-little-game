use bevy::prelude::*;
use bevy::app::AppExit;
use rand::prelude::*;

// Game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

// Components
#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(Component)]
struct Star {
    speed: f32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct LivesText;

#[derive(Component)]
struct MenuText;

#[derive(Component)]
struct GameOverText;

#[derive(Component)]
struct InstructionsText;

// Resources
#[derive(Resource)]
struct GameScore {
    value: u32,
    lives: u32,
    stars_missed: u32,
}

#[derive(Resource)]
struct StarSpawnTimer {
    timer: Timer,
}

#[derive(Resource)]
struct GameDifficulty {
    star_speed: f32,
    spawn_rate: f32,
}

// Systems
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🌟 Catch the Falling Stars! 🌟".into(),
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .insert_resource(GameScore { value: 0, lives: 3, stars_missed: 0 })
        .insert_resource(StarSpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .insert_resource(GameDifficulty {
            star_speed: 100.0,
            spawn_rate: 1.0,
        })
        // Menu systems
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        // Playing systems
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(Update, (
            player_movement,
            spawn_stars,
            move_stars,
            check_star_collection,
            check_star_missed,
            update_ui,
            increase_difficulty,
            check_game_over,
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnExit(GameState::Playing), cleanup_game)
        // Game Over systems
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(Update, game_over_input.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
        .run();
}

// Menu Systems
fn setup_menu(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn((
        TextBundle::from_section(
            "🌟 CATCH THE FALLING STARS! 🌟",
            TextStyle {
                font: Default::default(),
                font_size: 48.0,
                color: Color::GOLD,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(150.0),
            left: Val::Px(50.0),
            ..default()
        }),
        MenuText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Use ← → arrows to move your basket\nCatch the falling stars to score points!\nDon't let 10 stars fall or you lose!\n\nPress SPACE to start",
            TextStyle {
                font: Default::default(),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(250.0),
            left: Val::Px(150.0),
            ..default()
        }),
        InstructionsText,
    ));
}

fn menu_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, Or<(With<MenuText>, With<InstructionsText>)>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

// Playing Systems
fn setup_game(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    mut difficulty: ResMut<GameDifficulty>,
) {
    // Reset game state
    score.value = 0;
    score.lives = 3;
    score.stars_missed = 0;
    difficulty.star_speed = 100.0;
    difficulty.spawn_rate = 1.0;

    // Spawn player (basket)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::ORANGE,
                custom_size: Some(Vec2::new(80.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -250.0, 0.0),
            ..default()
        },
        Player { speed: 300.0 },
    ));

    // Spawn UI
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: Default::default(),
                font_size: 32.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        ScoreText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "Lives: 3",
            TextStyle {
                font: Default::default(),
                font_size: 32.0,
                color: Color::RED,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        LivesText,
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in player_query.iter_mut() {
        let mut direction = 0.0;
        
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }
        
        let movement = direction * player.speed * time.delta_seconds();
        transform.translation.x += movement;
        
        // Keep player on screen
        transform.translation.x = transform.translation.x.clamp(-360.0, 360.0);
    }
}

fn spawn_stars(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<StarSpawnTimer>,
    difficulty: Res<GameDifficulty>,
) {
    timer.timer.set_duration(std::time::Duration::from_secs_f32(1.0 / difficulty.spawn_rate));
    
    if timer.timer.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-350.0..350.0);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 300.0, 0.0),
                ..default()
            },
            Star { speed: difficulty.star_speed },
        ));
    }
}

fn move_stars(
    time: Res<Time>,
    mut star_query: Query<(&mut Transform, &Star)>,
) {
    for (mut transform, star) in star_query.iter_mut() {
        transform.translation.y -= star.speed * time.delta_seconds();
    }
}

fn check_star_collection(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    player_query: Query<&Transform, (With<Player>, Without<Star>)>,
    star_query: Query<(Entity, &Transform), (With<Star>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            
            if distance < 50.0 {
                commands.entity(star_entity).despawn();
                score.value += 10;
            }
        }
    }
}

fn check_star_missed(
    mut commands: Commands,
    mut score: ResMut<GameScore>,
    star_query: Query<(Entity, &Transform), With<Star>>,
) {
    for (star_entity, star_transform) in star_query.iter() {
        if star_transform.translation.y < -320.0 {
            commands.entity(star_entity).despawn();
            score.stars_missed += 1;
            if score.lives > 0 {
                score.lives -= 1;
            }
        }
    }
}

fn update_ui(
    score: Res<GameScore>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<LivesText>)>,
    mut lives_query: Query<&mut Text, (With<LivesText>, Without<ScoreText>)>,
) {
    if let Ok(mut text) = score_query.get_single_mut() {
        text.sections[0].value = format!("Score: {}", score.value);
    }
    
    if let Ok(mut text) = lives_query.get_single_mut() {
        text.sections[0].value = format!("Lives: {}", score.lives);
    }
}

fn increase_difficulty(
    mut difficulty: ResMut<GameDifficulty>,
    score: Res<GameScore>,
) {
    // Increase difficulty based on score
    let level = score.value / 50;
    difficulty.star_speed = 100.0 + (level as f32 * 20.0);
    difficulty.spawn_rate = 1.0 + (level as f32 * 0.2);
}

fn check_game_over(
    score: Res<GameScore>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if score.lives == 0 {
        next_state.set(GameState::GameOver);
    }
}

fn cleanup_game(
    mut commands: Commands,
    entities_query: Query<Entity, Or<(With<Player>, With<Star>, With<ScoreText>, With<LivesText>)>>,
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn();
    }
}

// Game Over Systems
fn setup_game_over(mut commands: Commands, score: Res<GameScore>) {
    commands.spawn((
        TextBundle::from_section(
            "🌟 GAME OVER! 🌟",
            TextStyle {
                font: Default::default(),
                font_size: 48.0,
                color: Color::RED,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(200.0),
            ..default()
        }),
        GameOverText,
    ));

    commands.spawn((
        TextBundle::from_section(
            format!("Final Score: {}\n\nPress R to restart\nPress ESC to quit", score.value),
            TextStyle {
                font: Default::default(),
                font_size: 24.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(280.0),
            left: Val::Px(250.0),
            ..default()
        }),
        GameOverText,
    ));
}

fn game_over_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        next_state.set(GameState::Menu);
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn cleanup_game_over(
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOverText>>,
) {
    for entity in game_over_query.iter() {
        commands.entity(entity).despawn();
    }
}