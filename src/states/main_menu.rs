use notan::prelude::{App, KeyCode};

use crate::state::{
    change_game_scene, initialize_game_scene, register_game_scene, GameAppState, GameScene,
    GameState,
};

use super::TITLE;

pub static MAIN_MENU: &str = "main_menu";

pub fn register(app: &mut App, app_state: &mut GameAppState) {
    register_game_scene(&mut app_state.scenes, MAIN_MENU, MainMenuScene::default());
    initialize_game_scene(MAIN_MENU, app, app_state);
}

struct MainMenuScene {
    x: i32,
}

impl Default for MainMenuScene {
    fn default() -> Self {
        Self { x: 7777 }
    }
}

impl GameScene for MainMenuScene {
    fn init(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("MainMenuScene init");
        self.x += 1;
    }

    fn enter(&mut self, _app: &mut App, state: &mut GameState) {
        println!("MainMenuScene enter");
        // build the initial main menu console content
        let con = &mut state.con;

        con.screen80x50();
        con.set_bgcolor(0);
        con.set_fgcolor(1 | 2 | 4 | 8);
        con.cls();

        let pic = vec![
            "CCPS Solutions Presents",
            "  ",
            " SSSS  AAA  W   W DDDD",
            "S     A   A W   W D   D",
            " SSS  AAAAA W W W D   D",
            "    S A   A W W W D   D",
            "SSSS  A   A WW WW DDDD",
            "  ",
            "  ",
            "Simple ASCII Walk-around Demo",
            "  ",
            "RPGDX 2008 ASCII Mini-RPG Contest",
            "  ",
            "http://www.ccpssolutions.com",
        ];

        for (index, &line) in pic.iter().enumerate() {
            let x = (((con.console.size.0 as usize) - line.len()) / 2) as i32;
            con.outchars(x, (index as i32) + 4, line);
        }
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        if app.keyboard.was_pressed(KeyCode::Escape) {
            change_game_scene(TITLE, state);
        }
    }

    fn exit(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("MainMenuScene exit");
    }
}
