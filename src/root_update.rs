use notan::prelude::{App, KeyCode};

use crate::{
    main_menu_update::main_menu_update,
    state::{GameState, State},
};

pub fn root_update(app: &mut App, state: &mut State) {
    match state.game_state {
        GameState::MainMenuState => {
            main_menu_update(app, state);
        }
        GameState::PlayState => {
            if app.keyboard.was_pressed(KeyCode::Escape) {
                state.game_state = GameState::MainMenuState;
            }
        }
    }
}
