use notan::{
    draw::{Draw, DrawTextSection},
    prelude::{App, Graphics},
};

use crate::{constants::LOGICAL_SIZE, state::State};

pub fn main_menu_draw(_app: &mut App, _gfx: &mut Graphics, state: &mut State, draw: &mut Draw) {
    draw.clear(state.colors[0]);

    let text = "SAWD";

    draw.text(&state.font, text)
        .size(32.0)
        .h_align_center()
        .v_align_middle()
        .position(LOGICAL_SIZE.x * 0.5, LOGICAL_SIZE.y * 0.25)
        .color(state.colors[2 | 8]);
}
