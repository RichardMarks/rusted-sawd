mod config;
mod constants;

mod character;
mod map_events;
mod maps;
mod obj;
mod root_draw;
mod root_setup;
mod root_update;
mod rusted_renderer;
mod script;
mod script_events;
mod state;
mod states;

use notan::draw::DrawConfig;

use crate::config::get_window_config;
use crate::root_draw::root_draw;
use crate::root_setup::root_setup;
use crate::root_update::root_update;

fn main() {
    println!("One More Time...🤞");
    let window_config = get_window_config();

    match notan::init_with(root_setup)
        .add_config(window_config)
        .add_config(DrawConfig)
        .update(root_update)
        .draw(root_draw)
        .build()
    {
        Ok(_) => {}
        Err(err) => eprintln!("{:?}", err),
    }
}
