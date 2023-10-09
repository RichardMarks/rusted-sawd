use notan::prelude::{App, KeyCode};

use crate::state::{GameState, State};

pub fn root_update(app: &mut App, state: &mut State) {
    match state.game_state {
        GameState::MainMenuState => {
            if app.keyboard.was_pressed(KeyCode::Escape) {
                app.exit()
            }
            if app.keyboard.was_pressed(KeyCode::Space) {
                state.game_state = GameState::PlayState;
            }
        }
        GameState::PlayState => {
            if app.keyboard.was_pressed(KeyCode::Escape) {
                state.game_state = GameState::MainMenuState;
            }
        }
    }
}
