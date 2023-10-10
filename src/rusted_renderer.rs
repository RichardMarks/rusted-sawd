use notan::{
    draw::{Draw, DrawShapes, DrawTextSection},
    prelude::{App, Graphics},
};
use rusted_console::{Coord, Rusted};

use crate::{constants::LOGICAL_SIZE, state::State};

pub fn calculate_render_scale(state: &mut State, columns: f32, rows: f32) {
    let (cell_width, cell_height) = (LOGICAL_SIZE.x / columns, LOGICAL_SIZE.y / rows);
    state.cell_width = cell_width;
    state.cell_height = cell_height;
}

pub fn rusted_renderer(
    // notan params
    np: (&mut Draw, &mut App, &mut Graphics, &State),
    // rusted params
    rp: (&Rusted,),
) {
    let (draw, app, gfx, state) = np;
    let con = rp.0;

    for (coord, character, attributes) in con {
        let (background_color, foreground_color) = attributes;
        let background_color: usize = background_color as usize;
        let foreground_color: usize = foreground_color as usize;

        draw_background((draw, app, gfx, state), coord, background_color);

        if character == ' ' {
            continue;
        }

        if character == '\0' {
            continue;
        }

        draw_character((draw, app, gfx, state), coord, character, foreground_color);
    }
}

fn draw_background(
    // notan params
    np: (&mut Draw, &mut App, &mut Graphics, &State),
    coord: Coord,
    color_index: usize,
) {
    let (draw, _app, _gfx, state) = np;
    if let Some(color) = state.colors.get(color_index) {
        let Coord(column, row) = coord;
        let cell_width = state.cell_width;
        let cell_height = state.cell_height;
        let x = state.cell_width * (column as f32);
        let y = state.cell_height * (row as f32);
        // render the background
        draw.rect((x, y), (cell_width, cell_height))
            .fill_color(*color);
    }
}

fn draw_character(
    // notan params
    np: (&mut Draw, &mut App, &mut Graphics, &State),
    coord: Coord,
    character: char,
    color_index: usize,
) {
    let (draw, _app, _gfx, state) = np;
    if let Some(color) = state.colors.get(color_index) {
        let Coord(column, row) = coord;

        let cell_width = state.cell_width;
        let cell_height = state.cell_height;
        let font_size: f32 = cell_width.min(cell_height);
        let x = state.cell_width * (column as f32);
        let y = state.cell_height * (row as f32);

        draw.text(&state.font, character.to_string().as_str())
            .size(font_size)
            .h_align_left()
            .v_align_top()
            .position(x, y)
            .color(*color);
    }
}
