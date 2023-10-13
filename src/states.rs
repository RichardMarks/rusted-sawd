mod main_menu;
mod title;

use notan::prelude::App;

use crate::state::{change_game_scene, GameAppState};

pub use main_menu::MAIN_MENU;
pub use title::TITLE;

pub fn register_states(app: &mut App, app_state: &mut GameAppState) {
    title::register(app, app_state);
    main_menu::register(app, app_state);

    // initial state
    change_game_scene(TITLE, &mut app_state.state);
    // (&state.game_state_entries, &mut state.next_game_state, TITLE);
}
