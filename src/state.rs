use notan::{
    draw::Font,
    prelude::{AppState, Color},
};
use rusted_console::Rusted;

pub enum GameState {
    MainMenuState,
    PlayState,
}

/// represents the whole runtime game state
#[derive(AppState)]
pub struct State {
    /// the current gamestate of the game, eg, main menu, battle, field, etc...
    pub game_state: GameState,
    /// the color palette the whole game will use
    pub colors: Vec<Color>,
    /// the font that will be used to render the game
    pub font: Font,
    /// the console backend - not to be confused with a notan backend.
    pub con: Rusted,
}
