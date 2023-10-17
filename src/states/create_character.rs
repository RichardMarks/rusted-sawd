use notan::prelude::{App, KeyCode};
use rusted_console::CharInfo;

use crate::{
    character::{roll_stats, CharacterClass, CharacterStats},
    state::{
        change_game_scene, initialize_game_scene, register_game_scene, GameAppState, GameScene,
        GameState,
    },
};

pub static CREATE_CHARACTER: &str = "create_character";

pub fn register(app: &mut App, app_state: &mut GameAppState) {
    register_game_scene(
        &mut app_state.scenes,
        CREATE_CHARACTER,
        CreateCharacterScene::default(),
    );
    initialize_game_scene(CREATE_CHARACTER, app, app_state);
}

enum CreationStage {
    ChooseClass,
    RollStats,
    Confirm,
    Confirmed,
}

struct CreateCharacterScene {
    selected_option: i32,
    stage: CreationStage,
    rolled_stats: Option<CharacterStats>,

    // x,y,w,h of the selected chr - for copying to roll stats display
    selected_chr: (i32, i32, i32, i32),
    selected_chr_buffer: Vec<CharInfo>,
}

impl Default for CreateCharacterScene {
    fn default() -> Self {
        Self {
            selected_option: 0,
            stage: CreationStage::ChooseClass,
            rolled_stats: None,

            selected_chr: (0, 0, 0, 0),
            selected_chr_buffer: vec![],
        }
    }
}

impl GameScene for CreateCharacterScene {
    fn init(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("CreateCharacterScene init");
    }

    fn enter(&mut self, _app: &mut App, state: &mut GameState) {
        println!("CreateCharacterScene enter");

        self.stage = CreationStage::ChooseClass;
        self.selected_option = 0;

        let con = &mut state.con;

        con.screen80x50();
        con.set_bgcolor(0);
        con.set_fgcolor(1 | 2 | 4 | 8);
        con.cls();

        self.redraw(state);
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        if app.keyboard.was_pressed(KeyCode::Escape) {
            app.exit();
        }

        match self.stage {
            CreationStage::ChooseClass => {
                if app.keyboard.was_pressed(KeyCode::Left) || app.keyboard.was_pressed(KeyCode::A) {
                    if self.selected_option > 0 {
                        self.selected_option -= 1;
                    } else if self.selected_option == 0 {
                        self.selected_option = 3;
                    }
                    self.redraw(state);
                }
                if app.keyboard.was_pressed(KeyCode::Right) || app.keyboard.was_pressed(KeyCode::D)
                {
                    if self.selected_option < 3 {
                        self.selected_option += 1;
                    } else if self.selected_option == 3 {
                        self.selected_option = 0;
                    }
                    self.redraw(state);
                }
                if app.keyboard.was_pressed(KeyCode::Space)
                    || app.keyboard.was_pressed(KeyCode::Return)
                {
                    let mut character_class = CharacterClass::Warrior;
                    match self.selected_option {
                        0 => {
                            println!("selected warrior class");
                            character_class = CharacterClass::Warrior;
                        }
                        1 => {
                            println!("selected dwarf class");
                            character_class = CharacterClass::Dwarf;
                        }
                        2 => {
                            println!("selected thief class");
                            character_class = CharacterClass::Thief;
                        }
                        3 => {
                            println!("selected mage class");
                            character_class = CharacterClass::Mage;
                        }
                        _ => {
                            // should not be possible
                        }
                    }

                    self.redraw(state);
                    self.selected_option = 0;

                    // copy the chr
                    self.selected_chr_buffer = vec![
                        CharInfo::default();
                        (self.selected_chr.2 * self.selected_chr.3)
                            as usize
                    ];
                    state.con.copy(
                        self.selected_chr.0,
                        self.selected_chr.1,
                        self.selected_chr.2,
                        self.selected_chr.3,
                        &mut self.selected_chr_buffer,
                    );

                    self.stage = CreationStage::RollStats;
                    self.rolled_stats = Some(roll_stats(character_class));
                    self.redraw(state);
                }
            }
            CreationStage::RollStats => {
                // choose from "Roll Again" or "Continue" buttons
                if app.keyboard.was_pressed(KeyCode::Left) || app.keyboard.was_pressed(KeyCode::A) {
                    if self.selected_option == 0 {
                        self.selected_option = 1;
                    } else {
                        self.selected_option = 0;
                    }
                    self.redraw(state);
                }
                if app.keyboard.was_pressed(KeyCode::Right) || app.keyboard.was_pressed(KeyCode::D)
                {
                    if self.selected_option == 0 {
                        self.selected_option = 1;
                    } else {
                        self.selected_option = 0;
                    }
                    self.redraw(state);
                    self.redraw(state);
                }
                if app.keyboard.was_pressed(KeyCode::Space)
                    || app.keyboard.was_pressed(KeyCode::Return)
                {
                    if self.selected_option == 0 {
                        self.rolled_stats = Some(roll_stats(
                            self.rolled_stats.clone().unwrap().character_class,
                        ));
                        self.redraw(state);
                    } else {
                        self.stage = CreationStage::Confirm;
                        self.redraw(state);
                    }
                }
            }
            CreationStage::Confirm => {
                // choose to accept the created character or restart the process
                if app.keyboard.was_pressed(KeyCode::Left) || app.keyboard.was_pressed(KeyCode::A) {
                    if self.selected_option == 0 {
                        self.selected_option = 1;
                    } else {
                        self.selected_option = 0;
                    }
                    self.redraw(state);
                }
                if app.keyboard.was_pressed(KeyCode::Right) || app.keyboard.was_pressed(KeyCode::D)
                {
                    if self.selected_option == 0 {
                        self.selected_option = 1;
                    } else {
                        self.selected_option = 0;
                    }
                    self.redraw(state);
                    self.redraw(state);
                }
                if app.keyboard.was_pressed(KeyCode::Space)
                    || app.keyboard.was_pressed(KeyCode::Return)
                {
                    if self.selected_option == 0 {
                        // restart the process
                        self.stage = CreationStage::ChooseClass;
                        state.con.screen80x50();
                        state.con.set_bgcolor(0);
                        state.con.set_fgcolor(1 | 2 | 4 | 8);
                        state.con.cls();
                        self.redraw(state);
                    } else {
                        // continue the game
                        self.stage = CreationStage::Confirmed;
                        self.redraw(state);
                    }
                }
            }
            CreationStage::Confirmed => {
                // take the created character and populate the game state
                // change to the in game scene
                let Some(stats) = self.rolled_stats.clone() else {
                    return;
                };

                state.player.steps = 0;
                state.player.level = 1;
                state.player.experience = 0;
                state.player.gold = 100;
                state.player.max_ap = 1;
                state.player.cur_ap = 1;

                state.player.max_hp = stats.hp;
                state.player.cur_hp = state.player.max_hp;

                state.player.max_mp = stats.mp;
                state.player.cur_mp = state.player.max_mp;

                state.player.attack = stats.attack;
                state.player.defense = stats.defense;
                state.player.strength = stats.strength;
                state.player.magic = stats.magic;

                state.player.print();

                app.exit();
            }
        }
    }

    fn exit(&mut self, _app: &mut App, _state: &mut GameState) {
        println!("CreateCharacterScene exit");
    }
}

static WARRIOR: [u32; 36] = [
    0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40407C40, 0x40404040, 0x40404040,
    0x4040407C, 0x40404040, 0x40404040, 0x2F5C4040, 0x7C404040, 0x40404040, 0x405C2F40, 0x40424040,
    0x40404040, 0x2F523E3C, 0x522F3B40, 0x40404040, 0x2F405C71, 0x702F4040, 0x40404040, 0x40624040,
    0x2D2D4040, 0x40404040, 0x40404040, 0x28292829, 0x40404040, 0x40404040, 0x40282928, 0x29404040,
    0x40404040, 0x407E7E40, 0x407E7E40, 0x404040FF,
];
static DWARF: [u32; 36] = [
    0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040,
    0x40404040, 0x40404040, 0x405F545F, 0x5F404040, 0x40404040, 0x40444040, 0x402F4040, 0x40404040,
    0x4040445F, 0x5F2F402F, 0x5C404040, 0x40404040, 0x407C4040, 0x5C2F4040, 0x40404040, 0x4040426D,
    0x523E3C52, 0x5C404040, 0x4040407C, 0x402F7170, 0x5C644040, 0x40404040, 0x7C5F3840, 0x40385F40,
    0x40404040, 0x40407E7E, 0x40407E7E, 0x404040FF,
];
static THIEF: [u32; 36] = [
    0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040, 0x40404040,
    0x407C5C40, 0x40404040, 0x40404040, 0x40407C40, 0x5C404040, 0x40404040, 0x402F5C7C, 0x40406240,
    0x40404040, 0x2D2D5C2F, 0x7C404054, 0x40404040, 0x40405C52, 0x3D2D2D6D, 0x42404040, 0x40404040,
    0x71707C40, 0x2F404040, 0x40404040, 0x2F40407C, 0x2F404040, 0x40404040, 0x79404040, 0x405C4040,
    0x40404040, 0x407E7E40, 0x40407E7E, 0x404040FF,
];
static MAGE: [u32; 36] = [
    0x40404040, 0x4040405C, 0x7C2F4040, 0x40404040, 0x4040402D, 0x2D302D2D, 0x40404040, 0x40402F5C,
    0x402F7C5C, 0x40404040, 0x40402F40, 0x405C407C, 0x40404040, 0x4040407E, 0x21217E40, 0x7C404040,
    0x40404040, 0x40575740, 0x407C4040, 0x40404040, 0x2F524040, 0x526D4240, 0x40404040, 0x407C5C5F,
    0x5F2F407C, 0x40404040, 0x4040622F, 0x40405C40, 0x7C404040, 0x40404040, 0x38575738, 0x407C4040,
    0x40404040, 0x407E7E7E, 0x7E404040, 0x404040FF,
];

impl CreateCharacterScene {
    fn redraw(&mut self, state: &mut GameState) {
        match self.stage {
            CreationStage::ChooseClass => self.redraw_choose_class(state),
            CreationStage::RollStats => self.redraw_roll_stats(state),
            CreationStage::Confirm => self.redraw_confirm(state),
            CreationStage::Confirmed => {}
        }
    }

    fn redraw_roll_stats(&mut self, state: &mut GameState) {
        let Some(stats) = self.rolled_stats.clone() else {
            return;
        };

        let con = &mut state.con;

        con.set_bgcolor(0);
        con.set_fgcolor(1 | 2 | 4);
        con.cls();
        // con.draw_window_fill(60, 0, 20, 50);
        con.draw_panel(0, 0, 19, 50);

        con.paste(
            2,
            2,
            self.selected_chr.2,
            self.selected_chr.3,
            &mut self.selected_chr_buffer,
        );

        con.draw_panel_ex(20, 5, 40, 40, (0, 4), (4, 0));

        con.set_bgcolor(4);
        con.set_fgcolor(1 | 2 | 4 | 8);

        con.outchars((80 - 15) / 2, 7, "Character Stats");

        let mut stat_y = 13;
        let stat_x = 23;

        con.outchars(
            stat_x,
            stat_y,
            format!("HP:       {:>25}", format!("{:4}", stats.hp)).as_str(),
        );
        stat_y += 2;
        con.outchars(
            stat_x,
            stat_y,
            format!("MP:       {:>25}", format!("{:4}", stats.mp)).as_str(),
        );
        stat_y += 4;
        con.outchars(
            stat_x,
            stat_y,
            format!("ATTACK:   {:>25}", format!("{:3}", stats.attack)).as_str(),
        );
        stat_y += 3;
        con.outchars(
            stat_x,
            stat_y,
            format!("DEFENSE:  {:>25}", format!("{:3}", stats.defense)).as_str(),
        );
        stat_y += 3;
        con.outchars(
            stat_x,
            stat_y,
            format!("STRENGTH: {:>25}", format!("{:3}", stats.strength)).as_str(),
        );
        stat_y += 3;
        con.outchars(
            stat_x,
            stat_y,
            format!("MAGIC:    {:>25}", format!("{:3}", stats.magic)).as_str(),
        );

        let btn_w = 14;
        let btn_h = 5;
        let btn_count = 2;
        let btn_spacing = 2;
        let display_width = (btn_w * btn_count) + (btn_spacing * (btn_count - 1));
        let display_x = ((con.console.size.0 as i32) - display_width) / 2;
        let display_y = 45 - (btn_h + btn_spacing);

        if self.selected_option == 0 {
            con.draw_button(
                (display_x, display_y, btn_w, btn_h),
                "Roll Again",
                1 | 2 | 4 | 8,
                4,
            );
            con.draw_button(
                (display_x + btn_w + btn_spacing, display_y, btn_w, btn_h),
                " Continue ",
                1 | 2 | 4,
                4,
            );
        } else {
            con.draw_button(
                (display_x, display_y, btn_w, btn_h),
                "Roll Again",
                1 | 2 | 4,
                4,
            );
            con.draw_button(
                (display_x + btn_w + btn_spacing, display_y, btn_w, btn_h),
                " Continue ",
                1 | 2 | 4 | 8,
                4,
            );
        }
    }

    fn redraw_confirm(&mut self, state: &mut GameState) {
        let con = &mut state.con;

        let btn_w = 14;
        let btn_h = 5;
        let btn_count = 2;
        let btn_spacing = 2;
        let display_width = (btn_w * btn_count) + (btn_spacing * (btn_count - 1));
        let display_x = ((con.console.size.0 as i32) - display_width) / 2;
        let display_y = 45 - (btn_h + btn_spacing);

        if self.selected_option == 0 {
            con.draw_button(
                (display_x, display_y, btn_w, btn_h),
                " Restart  ",
                1 | 2 | 4 | 8,
                4,
            );
            con.draw_button(
                (display_x + btn_w + btn_spacing, display_y, btn_w, btn_h),
                " Continue ",
                1 | 2 | 4,
                4,
            );
        } else {
            con.draw_button(
                (display_x, display_y, btn_w, btn_h),
                " Restart  ",
                1 | 2 | 4,
                4,
            );
            con.draw_button(
                (display_x + btn_w + btn_spacing, display_y, btn_w, btn_h),
                " Continue ",
                1 | 2 | 4 | 8,
                4,
            );
        }
    }

    fn redraw_choose_class(&mut self, state: &mut GameState) {
        let con = &mut state.con;

        let num_icons_to_draw = 4;
        let icon_width = 13;
        let icon_height = 11;

        let panel_width = icon_width + 2;
        let panel_height = icon_height + 2;

        let button_width = 13;
        let button_height = 3;

        let panel_spacing = 4;

        let total_display_width =
            (panel_width * num_icons_to_draw) + (panel_spacing * (num_icons_to_draw - 1));

        let total_display_height = panel_height + panel_spacing + button_height;

        let display_x = ((con.console.size.0 as i32) - total_display_width) / 2;
        let display_y = ((con.console.size.1 as i32) - total_display_height) / 2;

        // con.draw_button((0, 0, 80, 50), "Create Your Character", 1|2|4|8, 4);

        let icons = vec![
            (" Warrior ", Vec::from(WARRIOR), self.selected_option == 0),
            ("  Dwarf  ", Vec::from(DWARF), self.selected_option == 1),
            ("  Thief  ", Vec::from(THIEF), self.selected_option == 2),
            ("  Mage   ", Vec::from(MAGE), self.selected_option == 3),
        ];

        let mut x = display_x;
        let y = display_y;

        for (button_label, chr_quads, is_selected) in icons.iter() {
            let mut panel_fill_bgc: u16 = 0;
            let mut panel_fill_fgc: u16 = 1 | 2 | 4 | 8;
            let mut panel_frame_bgc: u16 = 0;
            let mut panel_frame_fgc: u16 = 1 | 2 | 4;
            let mut chr_bgc: u16 = 0;
            let mut chr_fgc: u16 = 1 | 2 | 4;
            let mut btn_bgc: u16 = 0;
            let mut btn_fgc: u16 = 1 | 2 | 4;

            if *is_selected {
                btn_fgc = 1 | 2 | 4 | 8;
                btn_bgc = 4;

                chr_fgc = 1 | 2 | 8;
                chr_bgc = 4;

                panel_fill_bgc = 4;
                panel_fill_fgc = 1 | 2 | 4 | 8;
                panel_frame_bgc = 1 | 2 | 4 | 8;
                panel_frame_fgc = 0;

                self.selected_chr = (x, y, panel_width, panel_height);
            }

            let panel_fill = (panel_fill_fgc, panel_fill_bgc);
            let panel_frame = (panel_frame_fgc, panel_frame_bgc);

            con.draw_panel_ex(x, y, panel_width, panel_height, panel_fill, panel_frame);
            con.set_fgcolor(chr_fgc);
            con.set_bgcolor(chr_bgc);
            con.draw_chr_from_quads(x + 1, y + 1, 0x40, (icon_width, icon_height), chr_quads);
            con.draw_button(
                (
                    x + 1,
                    y + panel_height + panel_spacing,
                    button_width,
                    button_height,
                ),
                &button_label,
                btn_fgc,
                btn_bgc,
            );

            x += panel_width + panel_spacing;
        }
    }
}
