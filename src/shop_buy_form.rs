use notan::prelude::{App, KeyCode};
use rusted_console::Window;

use crate::{
    item::{GameItem, GameItemType},
    script::GameScript,
    shop::{get_cursor_direction, CursorDirection, ItemDetailsWindow, Shop, ShopType},
    state::GameState,
};

enum BuyFormStatus {
    SetupForm,
    ChooseItems,
    ShowSelectedItemDetails,
    ViewSelectedItemDetails,
    AcceptChosenItems,
    CalculateTotal,
    ShowTotal,
    AskPurchaseConfirmation,
    GetPurchaseConfirmationAnswer,
    NotEnoughMoney,
    ShowPurchase,
    NoPurchase,
    CancelBuy,
    AskLeaveShop,
    GetLeaveShopAnswer,
    Done,
}

pub struct ShopBuyForm {
    status: BuyFormStatus,
    cursor_x: i32,
    cursor_y: i32,

    window: Option<Window>,
    item_details_window: Option<ItemDetailsWindow>,

    num_items: i32,
    shop_type: ShopType,
    item_qty: Vec<i32>,
    items_available: Vec<GameItem>,
}

impl GameScript for ShopBuyForm {
    fn init(&mut self, app: &mut App, state: &mut GameState) {
        //
    }

    fn update(&mut self, app: &mut App, state: &mut GameState) {
        match self.status {
            BuyFormStatus::SetupForm => {
                self.setup_form(state);
                self.status = BuyFormStatus::ChooseItems;
            }
            BuyFormStatus::ChooseItems => {
                self.update_cursor(app, state);
                if app.keyboard.was_pressed(KeyCode::Space)
                    || app.keyboard.was_pressed(KeyCode::Return)
                {
                    if self.cursor_x == 1 {
                        self.status = BuyFormStatus::AcceptChosenItems;
                    } else if self.cursor_x == 2 {
                        self.status = BuyFormStatus::CancelBuy;
                    }
                }
            }
            BuyFormStatus::ShowSelectedItemDetails => {
                // open the item details window and item detail display
                let item_id: String = String::from("SELECTED_ITEM");
                self.item_details_window = Some(ItemDetailsWindow::new(item_id, state));
                self.status = BuyFormStatus::ViewSelectedItemDetails;
            }
            BuyFormStatus::ViewSelectedItemDetails => {
                if let Some(idw) = self.item_details_window.as_mut() {
                    idw.update(app, state);
                    if !idw.is_open {
                        self.item_details_window = None;
                        self.status = BuyFormStatus::ChooseItems;
                    }
                }
            }

            BuyFormStatus::AcceptChosenItems => todo!(),
            BuyFormStatus::CalculateTotal => todo!(),
            BuyFormStatus::ShowTotal => todo!(),
            BuyFormStatus::AskPurchaseConfirmation => todo!(),
            BuyFormStatus::GetPurchaseConfirmationAnswer => todo!(),
            BuyFormStatus::NotEnoughMoney => todo!(),
            BuyFormStatus::ShowPurchase => todo!(),
            BuyFormStatus::NoPurchase => todo!(),
            BuyFormStatus::CancelBuy => todo!(),
            BuyFormStatus::AskLeaveShop => todo!(),
            BuyFormStatus::GetLeaveShopAnswer => todo!(),
            BuyFormStatus::Done => todo!(),
        }
    }
}

impl ShopBuyForm {
    pub fn new(shop: &mut Shop) -> Self {
        Self {
            status: BuyFormStatus::SetupForm,
            cursor_x: 0,
            cursor_y: 0,
            window: None,
            item_details_window: None,
            num_items: shop.shop_keeper.items.len() as i32,
            shop_type: shop.shop_keeper.shop_type,
            item_qty: vec![0; shop.shop_keeper.items.len()],
            items_available: shop.shop_keeper.items.to_vec(),
        }
    }

    fn setup_form(&mut self, state: &mut GameState) {}

    fn update_cursor(&mut self, app: &mut App, state: &mut GameState) {
        let cursor_direction = get_cursor_direction(app);
        match cursor_direction {
            CursorDirection::Up => {
                if self.cursor_x == 0 {
                    if self.cursor_y > 0 {
                        self.cursor_y -= 1;
                    } else if self.cursor_y <= 0 {
                        self.cursor_y = self.num_items - 1;
                    }
                } else {
                    self.cursor_x = 0;
                    self.cursor_y = self.num_items - 1;
                }
            }
            CursorDirection::Down => {
                if self.cursor_x == 0 {
                    if self.cursor_y < self.num_items - 1 {
                        self.cursor_y += 1;
                    } else if self.cursor_y >= self.num_items - 1 {
                        self.cursor_x = 1;
                    }
                }
            }
            CursorDirection::Left => {
                if self.cursor_x == 0 {
                    if self.item_qty[self.cursor_y as usize] > 0 {
                        self.item_qty[self.cursor_y as usize] -= 1;
                    }
                } else {
                    if self.cursor_x == 1 {
                        self.cursor_x = 2;
                    } else if self.cursor_x == 2 {
                        self.cursor_x = 1;
                    }
                }
            }
            CursorDirection::Right => {
                if self.cursor_x == 0 {
                    match self.shop_type {
                        ShopType::Item => {
                            match self.items_available[self.cursor_y as usize].item_type {
                                GameItemType::Consumable => {
                                    if self.item_qty[self.cursor_y as usize] < 255 {
                                        self.item_qty[self.cursor_y as usize] += 1;
                                    }
                                }
                                GameItemType::Equipment => {
                                    // can we use this equipment?
                                    // if self.items_available[self.cursor_y as usize]
                                    //     .character_class
                                    //     .cmp(&state.player.base_stats.character_class)
                                    //     == std::cmp::Ordering::Equal
                                    // {
                                    //     // can use
                                    // } else {
                                    //     // cannot use
                                    // }
                                }
                            }
                        }
                        ShopType::Magic => {
                            // only allow magic to be purchased once
                            // TODO
                        }
                    }
                } else {
                    if self.cursor_x == 1 {
                        self.cursor_x = 2;
                    } else if self.cursor_x == 2 {
                        self.cursor_x = 1;
                    }
                }
            }
            CursorDirection::NoMovement => {}
        }
    }
}
