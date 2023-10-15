use notan::prelude::{App, KeyCode};

use crate::{
    maps::{DUSHAL_WEST_MAP_ID, MAPS},
    state::{
        change_game_scene, initialize_game_scene, register_game_scene, GameAppState, GameScene,
        GameState,
    },
    states::{IN_GAME, NAME_CHARACTER},
};

use super::TITLE;

pub static MAIN_MENU: &str = "main_menu";

pub fn register(app: &mut App, app_state: &mut GameAppState) {
    register_game_scene(&mut app_state.scenes, MAIN_MENU, MainMenuScene::default());
    initialize_game_scene(MAIN_MENU, app, app_state);
}

#[derive(PartialEq)]
enum MainMenuOption {
    NewGame,
    LoadGame,
    Credits,
    QuitGame,
}

struct MainMenuScene {
    x: i32,
    active_menu_option: MainMenuOption,
    menu_options: Vec<(MainMenuOption, i32, i32, i32, i32, String)>,
}

impl Default for MainMenuScene {
    fn default() -> Self {
        Self {
            x: 7777,
            active_menu_option: MainMenuOption::NewGame,
            menu_options: vec![],
        }
    }
}

impl GameScene for MainMenuScene {
    fn init(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("MainMenuScene init");
        self.x += 1;

        self.menu_options = vec![
            (
                MainMenuOption::NewGame,
                0,
                20,
                13,
                3,
                "New Game".to_string(),
            ),
            (
                MainMenuOption::LoadGame,
                14,
                20,
                13,
                3,
                "Load Game".to_string(),
            ),
            (
                MainMenuOption::Credits,
                29,
                20,
                13,
                3,
                "Credits".to_string(),
            ),
            (
                MainMenuOption::QuitGame,
                43,
                20,
                13,
                3,
                "Quit Game".to_string(),
            ),
        ];
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
            " SSSS  AAA  W   W DDDD ",
            "S     A   A W   W D   D",
            " SSS  AAAAA W W W D   D",
            "    S A   A W W W D   D",
            "SSSS  A   A WW WW DDDD ",
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

        self.draw_main_menu_buttons(state);
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        if app.keyboard.was_pressed(KeyCode::Escape) {
            change_game_scene(TITLE, state);
        }

        if app.keyboard.was_pressed(KeyCode::Left) {
            self.prev();
            self.draw_main_menu_buttons(state);
        }

        if app.keyboard.was_pressed(KeyCode::Right) {
            self.next();
            self.draw_main_menu_buttons(state);
        }

        if app.keyboard.was_pressed(KeyCode::Space) {
            self.select(app, state);
        }

        if app.keyboard.was_pressed(KeyCode::Return) {
            self.select(app, state);
        }
    }

    fn exit(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("MainMenuScene exit");
    }
}

impl MainMenuScene {
    fn prev(&mut self) {
        self.active_menu_option = match self.active_menu_option {
            MainMenuOption::NewGame => MainMenuOption::QuitGame,
            MainMenuOption::LoadGame => MainMenuOption::NewGame,
            MainMenuOption::Credits => MainMenuOption::LoadGame,
            MainMenuOption::QuitGame => MainMenuOption::Credits,
        }
    }

    fn next(&mut self) {
        self.active_menu_option = match self.active_menu_option {
            MainMenuOption::NewGame => MainMenuOption::LoadGame,
            MainMenuOption::LoadGame => MainMenuOption::Credits,
            MainMenuOption::Credits => MainMenuOption::QuitGame,
            MainMenuOption::QuitGame => MainMenuOption::NewGame,
        }
    }

    fn select(&mut self, app: &mut App, state: &mut GameState) {
        match self.active_menu_option {
            MainMenuOption::NewGame => {
                println!("New Game!");

                state.player.x = 8;
                state.player.y = 9;
                state.player.image = '\u{263A}';
                state.player.name = "Hero".to_string();

                state.current_map = Some(MAPS[DUSHAL_WEST_MAP_ID].chars().collect());

                change_game_scene(NAME_CHARACTER, state);
            }
            MainMenuOption::LoadGame => {
                println!("Load Game!");
            }
            MainMenuOption::Credits => {
                println!("Credits!");
            }
            MainMenuOption::QuitGame => {
                println!("Quit Game!");
                app.exit();
            }
        }
    }

    fn draw_main_menu_buttons(&mut self, state: &mut GameState) {
        let con = &mut state.con;

        let x = (((con.console.size.0 as usize) - 56) / 2) as i32;

        for opt in self.menu_options.iter() {
            if opt.0 == self.active_menu_option {
                // active
                con.draw_button(
                    (x + opt.1, opt.2, opt.3, opt.4),
                    opt.5.as_str(),
                    1 | 2 | 4 | 8,
                    4,
                );
            } else {
                // inactive
                con.draw_button(
                    (x + opt.1, opt.2, opt.3, opt.4),
                    opt.5.as_str(),
                    1 | 2 | 4,
                    0,
                );
            }
        }
    }
}
