use crate::constants::{MIN_WINDOW_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use notan::prelude::WindowConfig;

pub fn get_window_config() -> WindowConfig {
    WindowConfig::new()
        .set_title("SAWD")
        .set_high_dpi(true)
        .set_resizable(true)
        .set_min_size(MIN_WINDOW_SIZE.0, MIN_WINDOW_SIZE.1)
        .set_multisampling(0)
        .set_vsync(true)
        .set_size(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
}
