use notan::{
    draw::CreateDraw,
    math::{vec2, vec3, Mat4, Vec2},
    prelude::{App, Graphics},
};
use rusted_console::Coord;

use crate::{
    constants::LOGICAL_SIZE,
    rusted_renderer::{calculate_render_scale, rusted_renderer},
    state::GameAppState,
};

pub fn root_draw(app: &mut App, gfx: &mut Graphics, app_state: &mut GameAppState) {
    let (w, h) = gfx.size();
    let dpi = app.window().dpi() as f32;
    let window_size: Vec2 = vec2((w as f32) * dpi, (h as f32) * dpi);
    let (projection, _) = calculate_final_projection_matrix_and_ratio(window_size, LOGICAL_SIZE);

    let mut draw = gfx.create_draw();

    draw.set_projection(Some(projection));

    let Coord(columns, rows) = app_state.state.con.console.size;
    calculate_render_scale(app_state, columns as f32, rows as f32);

    let con = &app_state.state.con;
    let rp = (con,);

    rusted_renderer(
        // notan params
        (&mut draw, app, gfx, app_state),
        // rusted params
        rp,
    );

    gfx.render(&draw);
}

fn calculate_ratio(window_size: Vec2, logical_size: Vec2) -> f32 {
    (window_size.x / logical_size.x).min(window_size.y / logical_size.y)
}

fn calculate_translation_matrix(window_size: Vec2, logical_size: Vec2, ratio: f32) -> Mat4 {
    let center_x: f32 = 0.5 * (window_size.x - logical_size.x * ratio);
    let center_y: f32 = 0.5 * (window_size.y - logical_size.y * ratio);
    Mat4::from_translation(vec3(center_x, center_y, 1.0))
}

fn calculate_scale_matrix(ratio: f32) -> Mat4 {
    Mat4::from_scale(vec3(ratio, ratio, 1.0))
}

fn calculate_projection_matrix(window_size: Vec2) -> Mat4 {
    Mat4::orthographic_rh_gl(0.0, window_size.x, window_size.y, 0.0, -1.0, 1.0)
}

fn calculate_final_projection_matrix_and_ratio(
    window_size: Vec2,
    logical_size: Vec2,
) -> (Mat4, f32) {
    let ratio: f32 = calculate_ratio(window_size, logical_size);
    let scale: Mat4 = calculate_scale_matrix(ratio);
    let translation: Mat4 = calculate_translation_matrix(window_size, logical_size, ratio);
    let projection: Mat4 = calculate_projection_matrix(window_size);
    (projection * translation * scale, ratio)
}
