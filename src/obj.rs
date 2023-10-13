#[derive(Default, Debug)]
pub struct Obj {
    pub x: i32,
    pub y: i32,

    pub image: char,
    pub name: String,

    pub chr_class: i32,

    pub max_ap: i32,
    pub cur_ap: i32,

    pub max_hp: i32,
    pub cur_hp: i32,

    pub max_mp: i32,
    pub cur_mp: i32,

    pub attack: i32,
    pub defense: i32,
    pub strength: i32,
    pub magic: i32,

    pub level: i32,
    pub steps: i32,

    pub experience: u64,
    pub gold: u64,

    pub battle_icon_w: i32,
    pub battle_icon_h: i32,
    pub battle_icon: String,

    pub drinks: i32,
    pub drunk: bool,

    pub items: Vec<ShopItem>,
    pub items_equipped: Vec<String>,
    pub spells: Vec<ShopSpell>,
}

impl Obj {
    pub fn move_in_direction(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn step(&mut self) {
        self.steps += 1;
    }

    pub fn sober_up(&mut self) -> bool {
        if !self.drunk {
            return true;
        }
        self.drinks -= 1;
        if self.drinks <= 0 {
            self.drinks = 0;
            self.drunk = false;
        }
        !self.drunk
    }
}

#[derive(Debug)]
pub struct ShopItem {}

#[derive(Debug)]
pub struct ShopSpell {}
