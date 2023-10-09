use notan::math::{vec2, Vec2};

// design game for 1280x800 px to have 16x16 px "tiles" at battle screen mode 80x50 chrs
pub const LOGICAL_SIZE: Vec2 = vec2(1280.0, 800.0);

// 1728x1080 has same aspect ratio as 1280x800
pub const WINDOW_WIDTH: f32 = 1728.0 * 0.5;
pub const WINDOW_HEIGHT: f32 = 1080.0 * 0.5;

pub const MIN_WINDOW_SIZE: (u32, u32) = ((WINDOW_WIDTH * 0.5) as u32, (WINDOW_HEIGHT * 0.5) as u32);
