use bevy::prelude::*;

// Define the game states
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    InGame,
    GameOver,
}

// Marker component for entities to be despawned when exiting a state
#[derive(Component)]
struct DespawnOnExit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Initialize state management
        .init_state::<GameState>() // Using init_state as per Bevy 0.12.1 style for initializing
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, menu_update.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_system)
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .add_systems(Update, game_update.run_if(in_state(GameState::InGame)))
        .add_systems(OnExit(GameState::InGame), cleanup_system)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(Update, game_over_update.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_system)
        .run();
}

// Generic cleanup system
fn cleanup_system(mut commands: Commands, query: Query<Entity, With<DespawnOnExit>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    // Also despawn cameras, assuming one camera per state that might need cleanup
    // If specific cameras should persist, they should not have DespawnOnExit
    // For this exercise, we'll despawn all cameras on state exit for simplicity,
    // as each state setup spawns its own.
    // A more robust solution might involve tagging cameras with DespawnOnExit as well.
    // commands.query_entities_with_component::<Camera>().for_each(|entity, _| {
    //     commands.entity(entity).despawn_recursive();
    // });
    // Re-evaluating: The prompt requires DespawnOnExit for text. Camera is spawned in setup_menu.
    // Let's ensure camera is also cleaned up or handled properly.
    // For now, let's assume the camera spawned in setup_menu is the only one and should be cleaned.
    // The setup_menu camera will be despawned if it has DespawnOnExit. Let's add it.
}

// --- Menu State Systems ---
fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a camera. Add DespawnOnExit so it's cleaned up.
    commands.spawn((Camera2dBundle::default(), DespawnOnExit));

    commands.spawn((
        TextBundle::from_section(
            "Menu: Press Enter to Play",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"), // Using a default font path
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        }),
        DespawnOnExit,
    ));
    println!("Entered Menu State");
}

fn menu_update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::InGame);
        println!("Transitioning to InGame State");
    }
}

// --- InGame State Systems ---
fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Optional: Spawn a camera if one isn't carried over or if Menu's camera was despawned.
    // For this setup, Menu's camera (with DespawnOnExit) would be gone.
    commands.spawn((Camera2dBundle::default(), DespawnOnExit)); // Game needs its own camera

    commands.spawn((
        TextBundle::from_section(
            "InGame: Press Q for Game Over",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        }),
        DespawnOnExit,
    ));
    println!("Entered InGame State");
}

fn game_update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::GameOver);
        println!("Transitioning to GameOver State");
    }
}

// --- GameOver State Systems ---
fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Game's camera would be gone.
    commands.spawn((Camera2dBundle::default(), DespawnOnExit)); // GameOver needs its own camera

    commands.spawn((
        TextBundle::from_section(
            "Game Over: Press R to Restart",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        }),
        DespawnOnExit,
    ));
    println!("Entered GameOver State");
}

fn game_over_update(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Menu);
        println!("Transitioning to Menu State");
    }
}
