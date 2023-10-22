use std::collections::HashMap;

use notan::prelude::{App, KeyCode};
use rusted_console::Window;

use crate::{
    item::{GameItem, GameItemType},
    script::{exit_game_script, run_game_script, GameScript},
    shop_buy_form::ShopBuyForm,
    state::GameState,
};

#[derive(Debug, Default, Clone, Copy)]
pub enum ShopType {
    #[default]
    Item,
    Magic,
}

#[derive(Debug, Default)]
pub struct ShopKeeper {
    pub name: String,
    pub shop_type: ShopType,
    pub items: Vec<GameItem>,
}

impl Clone for ShopKeeper {
    fn clone(&self) -> Self {
        let mut items = vec![];
        for item in &self.items {
            items.push(GameItem {
                name: item.name.clone(),
                item_type: item.item_type.clone(),
                character_class: item.character_class.clone(),
                modifiers: item.modifiers.clone(),
            })
        }
        Self {
            name: self.name.clone(),
            shop_type: self.shop_type,
            items,
        }
    }
}

impl ShopKeeper {
    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn set_type(&mut self, shop_type: ShopType) {
        self.shop_type = shop_type;
    }

    pub fn push(&mut self, item_key: &str) {
        // does nothing right now
        println!(
            "adding item key {} to shop keeper {} inventory",
            item_key, self.name
        );
    }
}

enum Status {
    Intro,
    Menu,
    Buy,
    BuyEnd,
    Sell,
    SellEnd,
    Done,
}

pub enum ShopResult {
    Buy { items: Vec<String> },
    Sell { items: Vec<String> },
    Exit,
}

type ButtonDataLabel = &'static str;
type ButtonDataRect = (i32, i32, i32, i32);
type ButtonData = (ButtonDataLabel, ButtonDataRect);

pub struct Shop {
    status: Status,
    window: Option<Window>,
    window_rect: (i32, i32, i32, i32),
    pub shop_keeper: ShopKeeper,
    buttons: Vec<ButtonData>,
    cursor_x: i32,
    purchases: Vec<String>,
    sales: Vec<String>,
}

impl Default for Shop {
    fn default() -> Self {
        Self {
            status: Status::Intro,
            window: None,
            window_rect: (0, 0, 0, 0),
            shop_keeper: ShopKeeper::default(),
            buttons: vec![],
            cursor_x: 0,
            purchases: vec![],
            sales: vec![],
        }
    }
}

impl GameScript for Shop {
    fn init(&mut self, _app: &mut App, _state: &mut GameState) {
        // 40 - (38 / 2), (25 / 2) - (14 / 2), 38, 14

        self.window_rect = (40 - (38 / 2), (25 / 2) - (14 / 2), 38, 14);

        let x = self.window_rect.0 + 2;
        let y = self.window_rect.1 + 2;

        self.buttons = vec![
            ("BUY", (x + 4, y + 6, 7, 3)),
            ("SELL", (x + 12, y + 6, 8, 3)),
            ("EXIT", (x + 21, y + 6, 8, 3)),
        ];

        /*
        int x = box[0] + 2;
        int y = box[1] + 2;
        int caption_x = x - 2 + (box[2] / 2) - strlen(shop_name) / 2;
        char* msg = "What would you like to do?";
        int msg_x = x - 2 + (box[2] / 2) - strlen(msg) / 2;
        */
        // self.display_x = self.window_rect.0 + 2;
        // self.display_y = self.window_rect.1 + 2;
        // self.caption_x = self.display_x - 2 + (self.window_rect.2 / 2) - self.shop_keeper.name.chars().collect::<Vec<char>>().len();
        // self.caption_y = self.display_y + 2;
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        match self.status {
            Status::Intro => {
                self.intro(state);
                self.status = Status::Menu;
                state.last_shop_result = Some(ShopResult::Exit);
            }
            Status::Menu => {
                self.clear_cursors(state);
                self.draw_cursors(state);
                self.update_controls(app, state);
            }
            Status::Buy => {
                run_game_script(app, state, ShopBuyForm::new(self));
                self.status = Status::BuyEnd;
            }
            Status::BuyEnd => {
                state.last_shop_result = Some(ShopResult::Buy {
                    items: self.purchases.clone(),
                });
                self.status = Status::Done;
            }
            Status::Sell => {
                println!("TODO: SHOP -> SELL");
                self.status = Status::SellEnd;
            }
            Status::SellEnd => {
                state.last_shop_result = Some(ShopResult::Sell {
                    items: self.sales.clone(),
                });
                self.status = Status::Done;
            }
            Status::Done => {
                self.close_window(state);
                exit_game_script(state);
            }
        }
    }
}

impl Shop {
    pub fn new(shop_keeper: &ShopKeeper) -> Self {
        let my_shopkeep = ShopKeeper {
            name: shop_keeper.name.to_owned(),
            shop_type: shop_keeper.shop_type,
            ..Default::default()
        };

        Self {
            status: Status::Intro,
            window: None,
            window_rect: (0, 0, 0, 0),
            shop_keeper: my_shopkeep,
            buttons: vec![],
            purchases: vec![],
            sales: vec![],
            cursor_x: 0,
        }
    }

    fn intro(&mut self, state: &mut GameState) {
        self.open_window(state);
        let (wx, wy, ww, _wh) = self.window_rect;
        let name_length = self.shop_keeper.name.chars().collect::<Vec<char>>().len();
        let (x, y) = (wx + (ww - (name_length as i32)) / 2, wy + 2);
        state.con.outchars(x, y, &self.shop_keeper.name);
        let msg = "What would you like to do?";
        let msg_length = msg.chars().collect::<Vec<char>>().len();
        let (x, y) = (wx + (ww - (msg_length as i32)) / 2, wy + 4);
        state.con.outchars(x, y, msg);
        self.draw_buttons(state);
    }

    fn open_window(&mut self, state: &mut GameState) {
        self.window = Some(
            state
                .con
                .open_window(self.window_rect, 1 | 2 | 4 | 8, 4, true),
        );
    }
    fn draw_form(&mut self, state: &mut GameState) {
        self.draw_buttons(state);
    }

    fn draw_buttons(&mut self, state: &mut GameState) {
        for btn in self.buttons.iter() {
            state.con.draw_button(btn.1, btn.0, 1 | 2 | 4 | 8, 4);
            state.con.outchar(btn.1 .0 + 3, self.window_rect.1 + 6, ' ');
        }
    }

    fn clear_cursors(&mut self, state: &mut GameState) {
        for btn in self.buttons.iter() {
            state.con.outchar(btn.1 .0 + 3, self.window_rect.1 + 6, ' ');
        }
    }

    fn draw_cursors(&mut self, state: &mut GameState) {
        for (index, btn) in self.buttons.iter().enumerate() {
            if self.cursor_x == (index as i32) {
                state
                    .con
                    .outchar(btn.1 .0 + 3, self.window_rect.1 + 6, '\u{25BC}');
            }
        }
    }

    fn update_controls(&mut self, app: &mut App, state: &mut GameState) {
        if app.keyboard.was_pressed(KeyCode::A) || app.keyboard.was_pressed(KeyCode::Left) {
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
            } else if self.cursor_x == 0 {
                self.cursor_x = 2;
            }
        } else if app.keyboard.was_pressed(KeyCode::D) || app.keyboard.was_pressed(KeyCode::Right) {
            if self.cursor_x < 2 {
                self.cursor_x += 1;
            } else if self.cursor_x == 2 {
                self.cursor_x = 0;
            }
        } else if app.keyboard.was_pressed(KeyCode::Escape)
            || app.keyboard.was_pressed(KeyCode::Back)
        {
            self.status = Status::Done;
        } else if app.keyboard.was_pressed(KeyCode::Space)
            || app.keyboard.was_pressed(KeyCode::Return)
        {
            self.status = match self.cursor_x {
                x if x == 0 => Status::Buy,
                x if x == 1 => Status::Sell,
                x if x == 2 => Status::Done,
                _ => Status::Done,
            };
        }
    }

    fn close_window(&mut self, state: &mut GameState) {
        if self.window.is_none() {
            return;
        }
        state.con.close_window(&self.window.take().unwrap());
    }
}

pub fn draw_shop_ext(state: &mut GameState) {
    let caption = format!("${:>18}", state.player.gold);
    // state.con.draw_button((60, 1, 19, 3), &caption, 1|2|4|8, 4);
    state.con.set_bgcolor(4);
    state.con.set_fgcolor(1 | 2 | 4 | 8);
    state.con.draw_panel(60, 1, 19, 3);
    state.con.outchars(61, 2, &caption);
}

pub fn hide_shop_ext(state: &mut GameState) {
    state.con.set_bgcolor(0);
    state.con.set_fgcolor(0);
    state.con.draw_panel(60, 1, 19, 3);
}

// util should be moved later

pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
    NoMovement,
}

pub fn get_cursor_direction(app: &mut App) -> CursorDirection {
    if app.keyboard.was_pressed(KeyCode::W) || app.keyboard.was_pressed(KeyCode::Up) {
        return CursorDirection::Up;
    } else if app.keyboard.was_pressed(KeyCode::S) || app.keyboard.was_pressed(KeyCode::Down) {
        return CursorDirection::Down;
    } else if app.keyboard.was_pressed(KeyCode::A) || app.keyboard.was_pressed(KeyCode::Left) {
        return CursorDirection::Left;
    } else if app.keyboard.was_pressed(KeyCode::D) || app.keyboard.was_pressed(KeyCode::Right) {
        return CursorDirection::Right;
    }
    CursorDirection::NoMovement
}

macro_rules! format_item_modifier {
    ($label:expr, $modifier:expr) => {{
        let absolute = ($modifier).abs();
        let sign = match $modifier.cmp(&0) {
            std::cmp::Ordering::Equal => '=',
            std::cmp::Ordering::Greater => '+',
            std::cmp::Ordering::Less => '-',
        };
        format!("{:>4} {sign}[{absolute:3}]", $label)
    }};
}

pub struct ItemDetailsWindow {
    pub is_open: bool,
    window: Option<Window>,
    item_id: String,
}

impl ItemDetailsWindow {
    pub fn new(item_id: String, state: &mut GameState) -> Self {
        let mut idw = Self {
            is_open: false,
            window: None,
            item_id,
        };
        idw.open(state);
        idw
    }

    fn open(&mut self, state: &mut GameState) {
        let rect = (20, 4, 40, 18);

        self.window = Some(state.con.open_window(rect, 1 | 2 | 4, 0, true));

        let mut x = rect.0 + 2;
        let mut y = rect.1 + 2;

        let item: GameItem = get_master_item(&self.item_id);

        let modifiers = vec![
            (item.modifiers.max_hp.text, item.modifiers.max_hp.value),
            (item.modifiers.max_mp.text, item.modifiers.max_mp.value),
            (item.modifiers.max_ap.text, item.modifiers.max_ap.value),
            (item.modifiers.attack.text, item.modifiers.attack.value),
            (item.modifiers.defense.text, item.modifiers.defense.value),
            (item.modifiers.strength.text, item.modifiers.strength.value),
            (item.modifiers.magic.text, item.modifiers.magic.value),
            (item.modifiers.hp.text, item.modifiers.hp.value),
            (item.modifiers.mp.text, item.modifiers.mp.value),
            (item.modifiers.ap.text, item.modifiers.ap.value),
        ];

        state
            .con
            .outchars(x, y, format!("Name: {}", item.name).as_str());
        y += 2;

        for (index, pair) in modifiers.iter().enumerate() {
            state.con.outchars(
                x,
                y + (index as i32),
                format_item_modifier!(pair.0, pair.1).as_str(),
            );
        }

        x = rect.0 + 15;
        y = rect.1 + 4;

        match item.item_type {
            GameItemType::Equipment => {
                state.con.outchars(x, y, "Type: Equipment");
                y += 1;
                // is usable?
                if state
                    .player
                    .base_stats
                    .character_class
                    .cmp(&item.character_class)
                    == std::cmp::Ordering::Equal
                {
                    state.con.outchars(x, y, "Usable: Yes");
                } else {
                    state.con.outchars(x, y, "Usable: No");
                }
                // y += 1;
            }
            GameItemType::Consumable => {
                state.con.outchars(x, y, "Type: Consumable");
                // y += 1;
            }
        }

        self.is_open = true;
    }

    pub fn update(&mut self, app: &mut App, state: &mut GameState) {
        if !self.is_open {
            return;
        }
        // pressing space or return should close item details window
        if app.keyboard.was_pressed(KeyCode::Space) || app.keyboard.was_pressed(KeyCode::Return) {
            self.close(state);
        }
    }

    fn close(&mut self, state: &mut GameState) {
        if !self.is_open {
            return;
        }
        if self.window.is_none() {
            return;
        }
        state.con.close_window(&self.window.take().unwrap());
        self.is_open = false;
    }
}

pub fn get_master_item(item_id: &str) -> GameItem {
    let mut mil: HashMap<String, GameItem> = HashMap::new();
    // todo - populate the mil
    let tpl = mil.get(item_id).unwrap();
    GameItem {
        name: tpl.name.to_string(),
        item_type: tpl.item_type.clone(),
        character_class: tpl.character_class.clone(),
        modifiers: tpl.modifiers.clone(),
    }
}
