use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::default());
        app.add_system(toggle_pause_system);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    GameRunning,
    Paused,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::GameRunning
    }
}

fn toggle_pause_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match app_state.current() {
            AppState::GameRunning => match app_state.push(AppState::Paused) {
                Ok(_) => {}
                Err(e) => warn!("Failed to pause: {:?}", e),
            },
            AppState::Paused => match app_state.pop() {
                Ok(_) => {}
                Err(e) => warn!("Failed to unpause: {:?}", e),
            },
        }
    }
}
