use std::collections::HashMap;

use notan::{
    draw::CreateFont,
    prelude::{App, Color, Graphics},
};

use rusted_console::{Rusted, RustedMessage};

use crate::{
    constants::{COLOR_PALETTE, WINDOW_HEIGHT, WINDOW_WIDTH},
    map_events::{register_game_events, MapEventManager},
    obj::Obj,
    script_events::register_scripts,
    state::{Choice, GameAppState, GameState},
    states::register_states,
};

pub fn root_setup(app: &mut App, gfx: &mut Graphics) -> GameAppState {
    center_application_window(app);

    let font = gfx
        .create_font(include_bytes!("assets/fonts/Px437_IBM_CGA.ttf"))
        .unwrap();

    let colors: Vec<Color> = COLOR_PALETTE
        .iter()
        .map(|color_hex| Color::from_hex(*color_hex))
        .collect();

    let con: Rusted = Rusted::new();

    // con.screen80x50();
    // con.outchars(1, 1, "Hello, World!");
    // con.draw_button((3, 3, 10, 5), "Button", 1 | 2 | 4 | 8, 2);

    let msg = RustedMessage::new(true);
    // msg.show(&mut con, vec!["This is a test", "of the RustedMessage"]);

    let mut app_state: GameAppState = GameAppState {
        scenes: HashMap::new(),
        script: vec![],
        state: GameState {
            next_scene: None,
            current_scene: None,
            con,
            current_map: None,
            current_map_id: 0,
            mem: MapEventManager::new(),
            player: Obj::default(),

            message_box: None,
            choice_box: None,
            last_selected_choice: Choice::Invalid,

            script_running: false,
            next_script: None,
            dirty: true,
        },
        colors,
        font,
        cell_width: 0.0,
        cell_height: 0.0,
        msg,
    };

    // *STATE_MANAGER.lock().unwrap() = Some(Box::new(StateManager::default()));

    register_game_events(&mut app_state);
    register_scripts(&mut app_state);

    register_states(app, &mut app_state);

    app_state
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
