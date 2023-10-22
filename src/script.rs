use notan::prelude::App;

use crate::state::GameState;

pub trait GameScript: 'static {
    fn init(&mut self, app: &mut App, state: &mut GameState);
    fn update(&mut self, app: &mut App, state: &mut GameState);
}

pub struct GameScriptObject {
    behavior: Box<dyn GameScript>,
}

impl GameScript for GameScriptObject {
    fn init(&mut self, app: &mut App, state: &mut GameState) {
        self.behavior.init(app, state);
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        self.behavior.update(app, state);
    }
}

pub fn run_game_script(app: &mut App, state: &mut GameState, script: impl GameScript) {
    println!("run_game_script");
    let mut item = GameScriptObject {
        behavior: Box::new(script),
    };
    item.behavior.init(app, state);
    state.next_script = Some(item);
}

pub fn exit_game_script(state: &mut GameState) {
    println!("exit_game_script");
    state.script_commands.push(GameScriptCommand::PopScript);
}

pub enum GameScriptCommand {
    UpdateParentScene,
    PopScript,
    PopAllScripts,
}

// pub type ScriptFunction = fn(&mut App, &mut GameState) -> ();
