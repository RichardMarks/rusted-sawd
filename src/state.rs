use std::collections::HashMap;

use notan::{
    draw::Font,
    prelude::{App, AppState, Color},
};
use rusted_console::{Rusted, RustedChoice, RustedMessage};

use crate::{
    map_events::{MapEventManager, ScriptFunction},
    obj::Obj,
    script::GameScriptObject,
};

#[derive(AppState)]
pub struct GameAppState {
    pub script: Vec<GameScriptObject>,
    pub scenes: HashMap<String, GameSceneObject>,
    pub state: GameState,

    /// the color palette the whole game will use
    pub colors: Vec<Color>,
    /// the font that will be used to render the game
    pub font: Font,

    /// used by the console rendering systems
    pub cell_width: f32,
    pub cell_height: f32,
    /// general purpose message
    pub msg: RustedMessage,
}

#[derive(Debug)]
pub enum Choice {
    Invalid,
    Valid(u8),
}

pub struct GameState {
    pub current_scene: Option<String>,
    pub next_scene: Option<String>,
    /// the console backend - not to be confused with a notan backend.
    pub con: Rusted,

    pub message_box: Option<RustedMessage>,
    pub choice_box: Option<RustedChoice>,
    pub last_selected_choice: Choice,

    pub current_map: Option<Vec<char>>,
    pub current_map_id: usize,
    pub mem: MapEventManager,

    pub player: Obj,

    pub script_running: bool,
    pub next_script: Option<GameScriptObject>,

    pub dirty: bool,
}

pub trait GameScene: 'static {
    /// occurs once at registration
    fn init(&mut self, app: &mut App, state: &mut GameState);
    /// occurs each time the state is made active
    fn enter(&mut self, app: &mut App, state: &mut GameState);
    /// occurs each frame the state is active
    fn update(&mut self, app: &mut App, state: &mut GameState);
    /// occurs each time the state is made inactive
    fn exit(&mut self, app: &mut App, state: &mut GameState);
}

pub struct GameSceneObject {
    behavior: Box<dyn GameScene>,
}

impl GameScene for GameSceneObject {
    fn init(&mut self, app: &mut App, state: &mut GameState) {
        self.behavior.init(app, state);
    }

    fn enter(&mut self, app: &mut App, state: &mut GameState) {
        self.behavior.enter(app, state);
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        self.behavior.update(app, state);
    }

    fn exit(&mut self, app: &mut App, state: &mut GameState) {
        self.behavior.exit(app, state);
    }
}

pub fn register_game_scene(
    scenes: &mut HashMap<String, GameSceneObject>,
    id: &str,
    scene: impl GameScene,
) {
    scenes.insert(
        id.into(),
        GameSceneObject {
            behavior: Box::new(scene),
        },
    );
}

pub fn initialize_game_scene(id: &str, app: &mut App, app_state: &mut GameAppState) {
    let entry = app_state.scenes.get_mut(&id.to_string());
    entry.unwrap().behavior.init(app, &mut app_state.state);
}

pub fn update_game_scene(app: &mut App, app_state: &mut GameAppState) {
    // if no next scene, abort
    let Some(next_scene_id) = app_state.state.next_scene.clone() else {
        return;
    };

    // exit current scene
    if let Some(current_scene_id) = &app_state.state.current_scene {
        if let Some(current_scene) = app_state.scenes.get_mut(current_scene_id) {
            current_scene.exit(app, &mut app_state.state);
        }
    }

    // enter next scene
    if let Some(next_scene) = app_state.scenes.get_mut(&next_scene_id) {
        next_scene.enter(app, &mut app_state.state);
    }

    // set the next scene as current
    app_state.state.current_scene = Some(next_scene_id);

    // clear the next scene
    app_state.state.next_scene = None;
}

pub fn change_game_scene(next_scene_id: &str, state: &mut GameState) {
    state.next_scene = Some(next_scene_id.to_string());
}
