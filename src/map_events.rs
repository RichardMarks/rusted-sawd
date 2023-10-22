pub static GAME_EVENT_TABLE: [u64; 24] = [
    0x574152508368885B,
    0x57415250842D06D0,
    0x5741525084A4814F,
    0x5741525080A70948,
    0x5741525080298570,
    0x5741525082C70071,
    0x57415250820C8377,
    0x5741525081CA8437,
    0x53435054836D0000,
    0x5343505483440001,
    0x53435054849F0002,
    0x5343505484230003,
    0x5343505484A70004,
    0x5343505484A68005,
    0x53435054842A8006,
    0x53435054827B8007,
    0x53435054827D8008,
    0x5343505482820009,
    0x53435054823F000A,
    0x534350548232800B,
    0x534350548257000C,
    0x53435054813F000D,
    0x53435054818B000E,
    0x534350548138000F,
];

const WARP_IDENTIFIER: u64 = 0x57415250u64;
const SCRIPT_IDENTIFIER: u64 = 0x53435054u64;

use std::{collections::HashMap, ops::Shr};

use notan::prelude::App;

use crate::state::{GameAppState, GameState};

pub type ScriptFunction = fn(&mut App, &mut GameState) -> ();

#[derive(Debug, Default)]
pub struct MapEventManager {
    // event_list: Vec<MapEvent>,
    warp_events_by_map: HashMap<u8, Vec<WarpModel>>,
    script_events_by_map: HashMap<u8, Vec<ScriptModel>>,
    scripts: HashMap<u16, ScriptFunction>,
}

impl MapEventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_script(&mut self, event_index: u16, script_function: ScriptFunction) {
        self.scripts.insert(event_index, script_function);
    }

    pub fn get_script(&self, event_index: u16) -> Option<&ScriptFunction> {
        self.scripts.get(&event_index)
    }

    pub fn add_warp_event(&mut self, encoded: u32) {
        let Some(warp) = decode_warp(encoded) else {
            return;
        };
        if !self.warp_events_by_map.contains_key(&warp.origin_map_id) {
            self.warp_events_by_map.insert(warp.origin_map_id, vec![]);
        }
        let warps = self
            .warp_events_by_map
            .get_mut(&warp.origin_map_id)
            .unwrap();
        warps.push(warp);
    }

    pub fn add_script_event(&mut self, encoded: u32) {
        let Some(script) = decode_script(encoded) else {
            return;
        };
        if !self
            .script_events_by_map
            .contains_key(&script.origin_map_id)
        {
            self.script_events_by_map
                .insert(script.origin_map_id, vec![]);
        }
        let scripts = self
            .script_events_by_map
            .get_mut(&script.origin_map_id)
            .unwrap();
        scripts.push(script);
    }

    pub fn warps_for_map_id(&self, map_id: usize) -> Vec<WarpModel> {
        let map_id: u8 = map_id as u8;
        let Some(warps) = self.warp_events_by_map.get(&map_id) else {
            return vec![];
        };
        warps.to_vec()
    }

    pub fn scripts_for_map_id(&self, map_id: usize) -> Vec<ScriptModel> {
        let map_id: u8 = map_id as u8;
        let Some(scripts) = self.script_events_by_map.get(&map_id) else {
            return vec![];
        };
        scripts.to_vec()
    }
}

pub fn register_game_events(app_state: &mut GameAppState) {
    let mem = &mut app_state.state.mem;

    for (index, game_event) in GAME_EVENT_TABLE.iter().enumerate() {
        let identifier: u64 = game_event.shr(32) & !0xFFFFFFFF00000000u64;
        match identifier {
            x if x == WARP_IDENTIFIER => {
                let encoded = (game_event & !0xFFFFFFFF00000000u64) as u32;
                // println!("ADD WARP   EVENT {:016X} {:08X} {:032b}", identifier, encoded, encoded);
                mem.add_warp_event(encoded);
            }
            x if x == SCRIPT_IDENTIFIER => {
                let encoded = (game_event & !0xFFFFFFFF00000000u64) as u32;
                // println!("ADD SCRIPT EVENT {:016X} {:08X} {:032b}", identifier, encoded, encoded);
                mem.add_script_event(encoded);
            }
            _ => {
                eprintln!(
                    "INVALID IDENTIFIER AT GAME EVENT #{} 0x{:016X} with {:064b}",
                    index, game_event, identifier
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct WarpModel {
    pub enabled: bool,
    pub once: bool,
    pub origin_map_id: u8,
    pub origin_x: u8,
    pub origin_y: u8,
    pub target_map_id: u8,
    pub target_x: u8,
    pub target_y: u8,
}

#[derive(Debug, Clone)]
pub struct ScriptModel {
    pub enabled: bool,
    pub once: bool,
    pub origin_map_id: u8,
    pub origin_x: u8,
    pub origin_y: u8,
    pub event_index: u16,
}

fn decode_warp(encoded: u32) -> Option<WarpModel> {
    let shr31 = (encoded & !0x7FFFFFFFu32).shr(0x1F);
    let shr30 = (encoded & !0xBFFFFFFFu32).shr(0x1E);
    let shr24 = (encoded & !0xC0FFFFFFu32).shr(0x18);
    let shr15 = (encoded & !0xFF007FFFu32).shr(0x0F);
    let shr09 = (encoded & !0xFFFF81FFu32).shr(0x09);
    let shr00 = encoded & !0xFFFFFE00u32;

    let enabled = match shr31 {
        x if x == 0u32 => false,
        x if x != 0u32 => true,
        _ => unreachable!("should not be possible"),
    };

    let once = match shr30 {
        x if x == 0u32 => false,
        x if x != 0u32 => true,
        _ => unreachable!("should not be possible"),
    };

    let origin_map_id = shr24 as u32 as u8;
    let origin_index = shr15;
    let origin_x = (origin_index % 30u32) as u8;
    let origin_y = (origin_index / 30u32) as u8;

    let target_map_id = shr09 as u32 as u8;
    let target_index = shr00;
    let target_x = (target_index % 30u32) as u8;
    let target_y = (target_index / 30u32) as u8;

    // validate

    let warp_model: WarpModel = WarpModel {
        enabled,
        once,
        origin_map_id,
        origin_x,
        origin_y,
        target_map_id,
        target_x,
        target_y,
    };
    Some(warp_model)
}

fn decode_script(encoded: u32) -> Option<ScriptModel> {
    let shr31 = (encoded & !0x7FFFFFFFu32).shr(0x1F);
    let shr30 = (encoded & !0xBFFFFFFFu32).shr(0x1E);
    let shr24 = (encoded & !0xC0FFFFFFu32).shr(0x18);
    let shr15 = (encoded & !0xFF007FFFu32).shr(0x0F);
    let shr00 = encoded & !0xFFFF8000u32;

    let enabled = match shr31 {
        x if x == 0u32 => false,
        x if x != 0u32 => true,
        _ => unreachable!("should not be possible"),
    };

    let once = match shr30 {
        x if x == 0u32 => false,
        x if x != 0u32 => true,
        _ => unreachable!("should not be possible"),
    };

    let origin_map_id = shr24 as u32 as u8;
    let origin_index = shr15;
    let origin_x = (origin_index % 30u32) as u8;
    let origin_y = (origin_index / 30u32) as u8;

    let event_index = shr00 as u16;

    // validate

    let script_model: ScriptModel = ScriptModel {
        enabled,
        once,
        origin_map_id,
        origin_x,
        origin_y,
        event_index,
    };
    Some(script_model)
}

/*

LIMITATION: maps are 30*14 each - map warp location is within 0 and 419 thus requires 9 bits to encode
LIMITATION: 64 maps possible   - target map is within 0 and 63 requiring 6 bits to encode
LIMITATION: 32767 scripted events possible (26880 would cover every cell of all 64 maps, leaving 5887 overlapping events possible)

script map event requires
    enable        1 bit    true | false
    once          1 bit    true | false
    origin_map_id 6 bits   0 to 63
    origin_x      9 bits   (x + y * 30) >= 0 < 420
    origin_y
    event_index   15 bits  0x0000 to 0x7fff


warp requires
    enable        1 bit
    once          1 bit
    origin_map_id 6 bits
    origin_x      9 bits
    origin_y
    target_map_id 6 bits
    target_x      9 bits
    target_y
    -------------------
                 32 bits

able to encode all event data in a single u32 value

*/
