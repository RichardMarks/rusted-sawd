use notan::prelude::{App, KeyCode};
use rusted_console::{Rusted, RustedMessage, Window};

use crate::state::{
    change_game_scene, initialize_game_scene, register_game_scene, GameAppState, GameScene,
    GameState,
};

use super::CREATE_CHARACTER;

pub static NAME_CHARACTER: &str = "name_character";

pub fn register(app: &mut App, app_state: &mut GameAppState) {
    register_game_scene(
        &mut app_state.scenes,
        NAME_CHARACTER,
        NameCharacterScene::default(),
    );
    initialize_game_scene(NAME_CHARACTER, app, app_state);
}

enum NamingStatus {
    Prompting,
    Entering,
    Entered,
}

struct NameCharacterScene {
    naming_status: NamingStatus,
    entry: Option<NameEntry>,
}

impl Default for NameCharacterScene {
    fn default() -> Self {
        Self {
            naming_status: NamingStatus::Prompting,
            entry: None,
        }
    }
}

impl GameScene for NameCharacterScene {
    fn init(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("NameCharacterScene init");
    }

    fn enter(&mut self, _app: &mut App, state: &mut GameState) {
        println!("NameCharacterScene enter");
        // build the initial main menu console content
        let con = &mut state.con;

        con.screen80x50();
        con.set_bgcolor(0);
        con.set_fgcolor(1 | 2 | 4 | 8);
        con.cls();

        let mut msg = RustedMessage::new(true);
        msg.show(
            con,
            vec![
                "Mysterious Voice:",
                "",
                "Before you may enter the World of Tiron,",
                "",
                "you must submit thy name.",
                "",
                "Your name may not be changed, so choose wisely.",
            ],
        );
        state.message_box = Some(msg);
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        match self.naming_status {
            NamingStatus::Prompting => {
                let mut entry = NameEntry::default();
                entry.show(&mut state.con);
                self.entry = Some(entry);
                self.naming_status = NamingStatus::Entering;
            }
            NamingStatus::Entering => {
                if let Some(entry) = self.entry.as_mut() {
                    entry.update(app, state);
                    if !entry.is_open {
                        self.naming_status = NamingStatus::Entered;
                    }
                }
            }
            NamingStatus::Entered => {
                if let Some(entry) = self.entry.as_mut() {
                    println!("entered name: {:?}", entry.name);
                    state.player.name = entry.name.to_owned();
                    self.entry = None;
                    // app.exit();
                    change_game_scene(CREATE_CHARACTER, state);
                }
            }
        }
    }

    fn exit(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("NameCharacterScene exit");
    }
}

type Btn<'a> = (&'a str, (i32, i32, i32, i32));

#[derive(Debug, Clone)]
struct NameEntry {
    is_open: bool,
    window: Option<Window>,
    cursor: (i32, i32),
    caret: i32,
    name: String,
    window_rect: (i32, i32, i32, i32),
    buttons: Vec<Btn<'static>>,
}

impl Default for NameEntry {
    fn default() -> Self {
        Self {
            is_open: false,
            window: None,
            cursor: (1, 6),
            caret: 4,
            name: String::from("Hero            "),
            window_rect: (0, 0, 0, 0),
            buttons: vec![],
        }
    }
}

impl NameEntry {
    pub fn show(&mut self, ctx: &mut Rusted) {
        let fgc: u16 = 1 | 2 | 4 | 8;
        let bgc: u16 = 4;

        let window_width = 37;
        let window_height = 18;
        let window_x = ((ctx.console.size.0 as i32) - window_width) / 2;
        let window_y = ((ctx.console.size.1 as i32) - window_height) / 2;
        self.window_rect = (window_x, window_y, window_width, window_height);

        let ax = self.window_rect.0 + 1;
        let ay = self.window_rect.1 + 2;
        self.buttons = vec![
            ("DELETE ", (ax + 0, ay + 12, 11, 3)),
            ("DEFAULT", (ax + 12, ay + 12, 11, 3)),
            ("ACCEPT ", (ax + 24, ay + 12, 11, 3)),
        ];

        self.window = Some(ctx.open_window(self.window_rect, fgc, bgc, true));
        self.is_open = true;
        self.redraw(ctx);
    }

    pub fn update(&mut self, app: &mut App, state: &mut GameState) {
        let ctx = &mut state.con;
        let mut close = false;
        if app.keyboard.was_pressed(KeyCode::Up) || app.keyboard.was_pressed(KeyCode::W) {
            // handle up
            if self.cursor.1 > 0 && self.cursor.1 != 6 {
                self.cursor.1 -= 1;
            } else if self.cursor.1 == 6 {
                self.cursor = (0, 4);
            }
            self.redraw(ctx);
        } else if app.keyboard.was_pressed(KeyCode::Down) || app.keyboard.was_pressed(KeyCode::S) {
            // handle down
            if self.cursor.1 < 4 {
                self.cursor.1 += 1;
            } else if self.cursor.1 == 4 {
                self.cursor = (0, 6);
            }
            self.redraw(ctx);
        } else if app.keyboard.was_pressed(KeyCode::Left) || app.keyboard.was_pressed(KeyCode::A) {
            // handle left
            if self.cursor.1 == 6 {
                // at buttons
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                } else if self.cursor.0 == 0 {
                    self.cursor.0 = 2;
                }
            } else {
                // at chr map
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                }
            }
            self.redraw(ctx);
        } else if app.keyboard.was_pressed(KeyCode::Right) || app.keyboard.was_pressed(KeyCode::D) {
            // handle right
            if self.cursor.1 == 6 {
                // at buttons
                if self.cursor.0 < 2 {
                    self.cursor.0 += 1;
                } else if self.cursor.0 == 2 {
                    self.cursor.0 = 0;
                }
            } else {
                // at chr map
                if self.cursor.0 < 12 {
                    self.cursor.0 += 1;
                }
            }
            self.redraw(ctx);
        } else if app.keyboard.was_pressed(KeyCode::Return)
            || app.keyboard.was_pressed(KeyCode::Space)
        {
            // handle confirm
            if self.cursor.1 == 6 {
                // at buttons
                match self.cursor.0 {
                    0 => {
                        // delete
                        if self.caret > 0 {
                            let mut name: Vec<char> = self.name.chars().collect();
                            self.caret -= 1;
                            name[self.caret as usize] = ' ';
                            self.name = name.into_iter().collect::<String>();
                            self.redraw(ctx);
                        }
                    }
                    1 => {
                        // default
                        self.name = String::from("Hero            ");
                        self.caret = 4;
                        self.redraw(ctx);
                    }
                    2 => {
                        // accept
                        self.name = self.name.trim().to_string();
                        close = true;
                    }
                    _ => {
                        // should not be possible
                    }
                }
            } else {
                // at chr map
                if self.caret < 15 {
                    let characters: Vec<char> =
                        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_! "
                            .chars()
                            .collect();
                    let mut name: Vec<char> = self.name.chars().collect();
                    let selected_character_index: usize =
                        (self.cursor.0 + (self.cursor.1 * 13)) as usize;
                    name[self.caret as usize] = characters[selected_character_index];
                    self.caret += 1;
                    self.name = name.into_iter().collect::<String>();
                    self.redraw(ctx);
                } else {
                    let mut msg = RustedMessage::new(true);
                    msg.show(ctx, vec!["Name length limit reached!"]);
                    state.message_box = Some(msg);
                }
            }
        } else if app.keyboard.was_pressed(KeyCode::Escape) {
            // handle esc close
            close = true;
        }

        if close {
            self.hide(ctx)
        }
    }

    pub fn hide(&mut self, ctx: &mut Rusted) {
        if self.is_open {
            if let Some(window) = &self.window {
                ctx.close_window(window);
                self.window = None;
                self.is_open = false;
            }
        }
    }

    fn redraw(&mut self, ctx: &mut Rusted) {
        self.draw_character_map(ctx);
        let name = self.name.to_owned();
        self.draw_name_entry_display(
            self.window_rect.0 + 1,
            self.window_rect.1 - 1,
            name.as_str(),
            ctx,
        );
        self.draw_buttons(ctx);
        self.clear_cursors(ctx);
        self.draw_cursors(ctx);
    }

    fn clear_cursors(&mut self, ctx: &mut Rusted) {
        let ax = self.window_rect.0 + 4;
        let ay = self.window_rect.1 + 4;
        for y in 0..5 {
            let dy = y * 2;
            for x in 0..14 {
                let dx = 1 + (x * 2);
                ctx.outchar(ax + dx, ay + dy, ' ');
            }
        }
        for btn in self.buttons.iter() {
            ctx.outchar(btn.1 .0 + 5, btn.1 .1 - 1, ' ');
        }
    }

    fn draw_cursors(&mut self, ctx: &mut Rusted) {
        if self.cursor.1 != 6 {
            // cursor is at chr map
            let ax = self.window_rect.0 + 4;
            let ay = self.window_rect.1 + 4;
            let dx = 1 + (self.cursor.0 * 2);
            let dy = self.cursor.1 * 2;
            ctx.set_fgcolor(1 | 2 | 8);
            ctx.outchar(ax + dx, ay + dy, '[');
            ctx.outchar(ax + dx + 2, ay + dy, ']');
            ctx.set_fgcolor(1 | 2 | 4 | 8);
        } else if self.cursor.1 == 6 {
            // cursor is at buttons
            let btn_at_cursor = &self.buttons[self.cursor.0 as usize];
            let btn_x = btn_at_cursor.1 .0 + 5;
            let btn_y = btn_at_cursor.1 .1 - 1;

            ctx.set_fgcolor(1 | 2 | 8);
            ctx.outchar(btn_x, btn_y, '\u{25BC}');
            ctx.set_fgcolor(1 | 2 | 4 | 8);
        }
    }

    fn draw_character_map(&mut self, ctx: &mut Rusted) {
        let ax = self.window_rect.0 + 6;
        let ay = self.window_rect.1 + 4;
        let characters: Vec<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_! "
                .chars()
                .collect();
        for y in 0..5 {
            let dy = y * 2;
            for x in 0..13 {
                let dx = x * 2;
                ctx.outchar(ax + dx, ay + dy, characters[(x + y * 13) as usize]);
            }
        }
    }

    fn draw_buttons(&mut self, ctx: &mut Rusted) {
        for btn in self.buttons.iter() {
            ctx.draw_button(btn.1, btn.0, 1 | 2 | 4 | 8, 4);
        }
    }

    fn draw_name_entry_display(&mut self, x: i32, y: i32, name: &str, ctx: &mut Rusted) {
        ctx.draw_button((x, y, 24, 4), name, 1 | 2 | 4 | 8, 4);
        ctx.set_fgcolor(1 | 2 | 8);
        ctx.outchar(x + 2 + self.caret, y + 2, '\u{25B2}');
        ctx.set_fgcolor(1 | 2 | 4 | 8);
    }
}
