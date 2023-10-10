use notan::{
    draw::CreateFont,
    prelude::{App, Color, Graphics},
};

use rusted_console::{Rusted, RustedMessage};

use crate::{
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    state::{GameState, State},
};

pub fn root_setup(app: &mut App, gfx: &mut Graphics) -> State {
    center_application_window(app);

    let font = gfx
        .create_font(include_bytes!("assets/fonts/Px437_IBM_CGA.ttf"))
        .unwrap();

    let colors: Vec<Color> = vec![
        // black
        Color::from_hex(0x000000FF),
        // dark colors
        Color::from_hex(0x7F0000FF),
        Color::from_hex(0x007F00FF),
        Color::from_hex(0x7F7F00FF),
        Color::from_hex(0x00007FFF),
        Color::from_hex(0x7F007FFF),
        Color::from_hex(0x007F7FFF),
        Color::from_hex(0x7F7F7FFF),
        // light colors
        Color::from_hex(0xF1F1F1FF),
        Color::from_hex(0xF10000FF),
        Color::from_hex(0x00F100FF),
        Color::from_hex(0xF1F100FF),
        Color::from_hex(0x0000F1FF),
        Color::from_hex(0xF100F1FF),
        Color::from_hex(0x00F1F1FF),
        // white
        Color::from_hex(0xFFFFFFFF),
    ];

    let mut con: Rusted = Rusted::new();

    con.screen80x50();
    con.outchars(1, 1, "Hello, World!");
    con.draw_button((3, 3, 10, 5), "Button", 1 | 2 | 4 | 8, 2);

    let mut msg = RustedMessage::new(true);
    msg.show(&mut con, vec!["This is a test", "of the RustedMessage"]);

    State {
        game_state: GameState::MainMenuState,
        colors,
        font,
        con,
        cell_width: 0.0,
        cell_height: 0.0,
        msg,
    }
}

fn center_application_window(app: &mut App) {
    let (screen_size, dpi) = get_host_screen_size_and_dpi(app);
    let (screen_width, screen_height) = screen_size;

    let center_x: i32 = ((screen_width - (WINDOW_WIDTH * dpi)) * 0.5) as i32;
    let center_y: i32 = ((screen_height - (WINDOW_HEIGHT * dpi)) * 0.5) as i32;

    app.window().set_position(center_x, center_y);
}

fn get_host_screen_size_and_dpi(app: &mut App) -> ((f32, f32), f32) {
    let (screen_width, screen_height) = app.window().container_size();
    let dpi = app.window().dpi() as f32;
    let screen_width: f32 = dpi * (screen_width as f32);
    let screen_height: f32 = dpi * (screen_height as f32);
    ((screen_width, screen_height), dpi)
}
