mod event_0000_dushal_player_home;
mod event_0001_dushal_magic_shop;

use notan::prelude::App;

use crate::{
    script::run_game_script,
    state::{GameAppState, GameState},
};

use self::event_0000_dushal_player_home::EventDushalPlayerHome;
use self::event_0001_dushal_magic_shop::EventDushalMagicShop;

pub fn event_dushal_player_home(app: &mut App, state: &mut GameState) {
    run_game_script(app, state, EventDushalPlayerHome::default());
}

pub fn event_dushal_magic_shop(app: &mut App, state: &mut GameState) {
    // run_game_script(app, state, Shop::default());
    run_game_script(app, state, EventDushalMagicShop::default());
}

pub fn event_dushal_inn(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_dushal_pub(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_dushal_armor_shop(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_dushal_weapon_shop(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_dushal_item_shop(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_inn(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_pub(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_armor_shop(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_magic_shop(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_weapon_shop(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_item_shop(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_thieves_hut(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_treasure_room(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn event_kirar_cave_entrance(_app: &mut App, _state: &mut GameState) {
    todo!()
}

pub fn register_scripts(app_state: &mut GameAppState) {
    let mem = &mut app_state.state.mem;

    let scripts = [
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
