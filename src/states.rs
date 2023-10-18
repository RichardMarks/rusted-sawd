mod create_character;
mod in_game;
mod main_menu;
mod name_character;
mod title;

use notan::prelude::App;

use crate::state::{change_game_scene, GameAppState};

pub use create_character::CREATE_CHARACTER;
pub use in_game::IN_GAME;
pub use main_menu::MAIN_MENU;
pub use name_character::NAME_CHARACTER;
pub use title::TITLE;

pub fn register_states(app: &mut App, app_state: &mut GameAppState) {
    title::register(app, app_state);
    main_menu::register(app, app_state);
    in_game::register(app, app_state);
    name_character::register(app, app_state);
    create_character::register(app, app_state);

    // initial state
    change_game_scene(TITLE, &mut app_state.state);
}
