// x, y, map_id
pub type MapEventLocation = (i32, i32, usize);

pub enum MapEvent {
    WarpEvent {
        from: MapEventLocation,
        to: MapEventLocation,
    },
    ScriptEvent {
        location: MapEventLocation,
        once: bool,
        enabled: bool,
    },
}
