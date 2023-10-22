use rusted_console::RustedMessage;

use crate::script::{exit_game_script, run_game_script, GameScript};

use crate::shop::{Shop, ShopKeeper, ShopResult, ShopType};

enum Status {
    Intro,
    GetShopResult,
    Done,
}

pub struct EventDushalMagicShop {
    status: Status,
    shop_keeper: ShopKeeper,
}

impl Default for EventDushalMagicShop {
    fn default() -> Self {
        Self {
            status: Status::Intro,
            shop_keeper: ShopKeeper::default(),
        }
    }
}

impl GameScript for EventDushalMagicShop {
    fn init(&mut self, _app: &mut notan::prelude::App, _state: &mut crate::state::GameState) {
        // initialize the shop keeper
        self.shop_keeper.set_name("DUSHAL MAGIC SHOP");
        self.shop_keeper.set_type(ShopType::Magic);

        self.shop_keeper.push("Burn");
        self.shop_keeper.push("Break");
    }

    fn update(&mut self, app: &mut notan::prelude::App, state: &mut crate::state::GameState) {
        match self.status {
            Status::Intro => {
                run_game_script(app, state, Shop::new(&self.shop_keeper));
                self.status = Status::GetShopResult;
            }
            Status::GetShopResult => {
                if let Some(shop_result) = &state.last_shop_result {
                    match shop_result {
                        ShopResult::Buy { items: _ } => {
                            let mut msg = RustedMessage::new(false);
                            msg.show(
                                &mut state.con,
                                vec!["Shop Keeper:", "Thanks for your business!"],
                            );
                            state.message_box = Some(msg);
                        }
                        ShopResult::Sell { items: _ } => {
                            let mut msg = RustedMessage::new(false);
                            msg.show(
                                &mut state.con,
                                vec!["Shop Keeper:", "Don't spend it all at once!"],
                            );
                            state.message_box = Some(msg);
                        }
                        ShopResult::Exit => {
                            let mut msg = RustedMessage::new(false);
                            msg.show(&mut state.con, vec!["Shop Keeper:", "Changed your mind?"]);
                            state.message_box = Some(msg);
                        }
                    }
                }
                state.last_shop_result = None;
                self.status = Status::Done;
            }
            Status::Done => {
                state.player.move_in_direction(0, 1);
                state.dirty = true;
                exit_game_script(state);
            }
        }
    }
}
