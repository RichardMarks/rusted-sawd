use notan::{
    draw::Draw,
    prelude::{App, Graphics},
};

use crate::{rusted_renderer::rusted_renderer, state::State};

pub fn main_menu_draw(app: &mut App, gfx: &mut Graphics, state: &mut State, draw: &mut Draw) {
    let con = &state.con;
    let rp = (con,);

    rusted_renderer(
        // notan params
        (draw, app, gfx, state),
        // rusted params
        rp,
    );

    // draw.clear(state.colors[0]);

    // let text = "SAWD";

    // draw.text(&state.font, text)
    //     .size(32.0)
    //     .h_align_center()
    //     .v_align_middle()
    //     .position(LOGICAL_SIZE.x * 0.5, LOGICAL_SIZE.y * 0.25)
    //     .color(state.colors[2 | 8]);
}
