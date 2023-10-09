use notan::prelude::{App, KeyCode};

use crate::state::{GameState, State};

pub fn main_menu_update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Escape) {
        app.exit()
    }
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.game_state = GameState::PlayState;
    }
}
