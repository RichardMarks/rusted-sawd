use notan::prelude::App;

use crate::state::{GameAppState, GameState};

pub fn event_dushal_player_home(app: &mut App, state: &mut GameState) {}

pub fn event_dushal_magic_shop(app: &mut App, state: &mut GameState) {}

pub fn event_dushal_inn(app: &mut App, state: &mut GameState) {}

pub fn event_dushal_pub(app: &mut App, state: &mut GameState) {}

pub fn event_dushal_armor_shop(app: &mut App, state: &mut GameState) {}

pub fn event_dushal_weapon_shop(app: &mut App, state: &mut GameState) {}

pub fn event_dushal_item_shop(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_inn(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_pub(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_armor_shop(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_magic_shop(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_weapon_shop(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_item_shop(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_thieves_hut(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_treasure_room(app: &mut App, state: &mut GameState) {}

pub fn event_kirar_cave_entrance(app: &mut App, state: &mut GameState) {}

pub fn register_scripts(app_state: &mut GameAppState) {
    let mem = &mut app_state.state.mem;

    let scripts = vec![
        event_dushal_player_home,
        event_dushal_magic_shop,
        event_dushal_inn,
        event_dushal_pub,
        event_dushal_armor_shop,
        event_dushal_weapon_shop,
        event_dushal_item_shop,
        event_kirar_inn,
        event_kirar_pub,
        event_kirar_armor_shop,
        event_kirar_magic_shop,
        event_kirar_weapon_shop,
        event_kirar_item_shop,
        event_kirar_thieves_hut,
        event_kirar_treasure_room,
        event_kirar_cave_entrance,
    ];

    for (index, script) in scripts.iter().enumerate() {
        mem.register_script(index as u16, *script);
    }
}
