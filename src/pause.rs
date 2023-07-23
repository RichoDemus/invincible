use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
        app.add_systems(Update, toggle_pause_system);
        app.add_systems(OnEnter(AppState::Paused), || info!("Paused"));
        app.add_systems(OnExit(AppState::Paused), || info!("Unpaused"));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    GameRunning,
    Paused,
}

fn toggle_pause_system(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match app_state.get() {
            AppState::GameRunning => next_state.set(AppState::Paused),
            AppState::Paused => next_state.set(AppState::GameRunning),
        }
    }
}
