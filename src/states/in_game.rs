use std::collections::HashMap;

use notan::prelude::{App, KeyCode};
use rusted_console::Rusted;

use crate::{
    obj::Obj,
    state::{
        change_game_scene, initialize_game_scene, register_game_scene, GameAppState, GameScene,
        GameState,
    }, maps::{WORLD_MAP, is_cell_empty, MAP_H, MAP_W},
};

use super::MAIN_MENU;

pub static IN_GAME: &str = "in_game";

pub fn register(app: &mut App, app_state: &mut GameAppState) {
    register_game_scene(&mut app_state.scenes, IN_GAME, InGameScene::default());
    initialize_game_scene(IN_GAME, app, app_state);
}

type Tile = (u16, u16, char);

struct InGameScene {
    x: i32,

    tileset: HashMap<char, Tile>,
    fallback_tile: Tile,

    t: f32,
}

impl Default for InGameScene {
    fn default() -> Self {
        Self {
            x: 7777,
            tileset: build_tileset(),
            fallback_tile: (2, 1 | 2 | 4 | 8, '?'),

            t: 0.0
        }
    }
}

impl GameScene for InGameScene {
    fn init(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("InGameScene init");
        self.x += 1;

        // state.current_map = Some(WORLD_MAP.chars().collect());
    }

    fn enter(&mut self, _app: &mut App, state: &mut GameState) {
        println!("InGameScene enter");
        self.draw_game_display(state);

        self.t = 0.0;
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        let player_motion_vector = self.get_player_motion_vector(app);

        if player_motion_vector.0 != 0.0 || player_motion_vector.1 != 0.0 {
            // we have player input
            self.t += app.timer.delta_f32();
            if self.t < 0.09 {
                return;
            }
            self.t = 0.0;
            
            if let Some(current_map) = state.current_map.clone() {
                let dx: i32 = player_motion_vector.0 as i32;
                let dy: i32 = player_motion_vector.1 as i32;
                let x: i32 = state.player.x + dx;
                let y: i32 = state.player.y + dy;
                if is_cell_empty(x, y, &current_map) {
                    state.player.move_in_direction(dx, dy);
                    self.draw_game_display(state);
                    state.player.step();
                    if state.player.sober_up() {
                        // sober!
                    }
                }
            }
        } else {
            self.t = 1.0;
        }

        if app.keyboard.was_pressed(KeyCode::Escape) {
            change_game_scene(MAIN_MENU, state);
        }
    }

    fn exit(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("InGameScene exit");
    }
}

impl InGameScene {
    fn get_player_motion_vector(&mut self, app: &mut App) -> (f32, f32) {
        let keyboard = &app.keyboard;
        let up = keyboard.is_down(KeyCode::W) || keyboard.is_down(KeyCode::Up);
        let down = keyboard.is_down(KeyCode::S) || keyboard.is_down(KeyCode::Down);
        let left = keyboard.is_down(KeyCode::A) || keyboard.is_down(KeyCode::Left);
        let right = keyboard.is_down(KeyCode::D) || keyboard.is_down(KeyCode::Right);

        let mut motion: (f32, f32) = (0.0, 0.0);

        if up {
            motion = (0.0, -1.0);
        } else if down {
            motion = (0.0, 1.0);
        } else if left {
            motion = (-1.0, 0.0);
        } else if right {
            motion = (1.0 , 0.0);
        }

        motion
    }

    fn draw_game_display(&mut self, state: &mut GameState) {
        self.draw_map(state);
        let con = &mut state.con;
        con.set_bgcolor(0);
        con.set_fgcolor(1 | 2 | 4 | 8);
        self.draw_obj(&state.player, con);
    }

    fn draw_map(&mut self, state: &mut GameState) {
        if let Some(current_map) = state.current_map.clone() {
            for y in 0..MAP_H {
                for x in 0..MAP_W {
                    let index: usize = (x as usize) + ((y as usize) * (MAP_W as usize));
                    let tile_id = current_map[index];
                    self.draw_tile(tile_id, x, y, state);
                }
            }
        }
    }

    fn draw_tile(&mut self, tile_id: char, x: i32, y: i32, state: &mut GameState) {
        let con = &mut state.con;
        if self.tileset.contains_key(&tile_id) {
            let (bgc, fgc, character) = self.tileset[&tile_id];
            con.set_bgcolor(bgc);
            con.set_fgcolor(fgc);
            con.outchar(x, y, character);
        } else {
            let (bgc, fgc, _character) = self.fallback_tile;
            con.set_bgcolor(bgc);
            con.set_fgcolor(fgc);
            con.outchar(x, y, tile_id);
        }
    }

    fn draw_obj(&mut self, obj: &Obj, con: &mut Rusted) {
        con.outchar(obj.x, obj.y, obj.image);
    }
}

fn build_tileset() -> HashMap<char, Tile> {
    let mut tileset: HashMap<char, Tile> = HashMap::new();

    // tree
    tileset.insert('T', (2, 2 | 8, '\u{2663}'));

    // water
    tileset.insert('W', (4, 4 | 8, '\u{2248}'));

    // house/town
    tileset.insert('H', (2, 1 | 2, '\u{2302}'));

    // grass
    tileset.insert(' ', (2, 1 | 2 | 4 | 8, ' '));

    // path
    tileset.insert('.', (1 | 2, 1 | 2 | 4, '.'));

    // wall
    tileset.insert('#', (1 | 2 | 4, 0, '\u{2592}'));

    tileset
}
