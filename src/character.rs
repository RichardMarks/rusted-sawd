use notan::prelude::{Random, Rng};

#[derive(Debug, Clone)]
pub enum CharacterClass {
    Warrior,
    Dwarf,
    Thief,
    Mage,
}

#[derive(Debug, Clone)]
pub struct CharacterStats {
    pub character_class: CharacterClass,
    pub hp: i32,
    pub mp: i32,
    pub attack: i32,
    pub defense: i32,
    pub strength: i32,
    pub magic: i32,
}

pub fn roll_stats(character_class: CharacterClass) -> CharacterStats {
    match character_class {
        CharacterClass::Warrior => roll_warrior_stats(),
        CharacterClass::Dwarf => roll_dwarf_stats(),
        CharacterClass::Thief => roll_thief_stats(),
        CharacterClass::Mage => roll_mage_stats(),
    }
}

fn get_random_i32(lower_bound: i32, upper_bound: i32) -> i32 {
    let mut rng = Random::default();
    rng.gen_range(lower_bound..=upper_bound) as i32
}

fn roll_warrior_stats() -> CharacterStats {
    let hp = get_random_i32(150, 275);
    let mp = 0;
    let attack = get_random_i32(4, 7);
    let defense = get_random_i32(3, 6);
    let strength = get_random_i32(2, 4);
    let magic = 0;

    CharacterStats {
        character_class: CharacterClass::Warrior,
        hp,
        mp,
        attack,
        defense,
        strength,
        magic,
    }
}

fn roll_dwarf_stats() -> CharacterStats {
    let hp = get_random_i32(100, 175);
    let mp = 0;
    let attack = get_random_i32(4, 7);
    let defense = get_random_i32(5, 8);
    let strength = get_random_i32(2, 4);
    let magic = 0;

    CharacterStats {
        character_class: CharacterClass::Dwarf,
        hp,
        mp,
        attack,
        defense,
        strength,
        magic,
    }
}

fn roll_thief_stats() -> CharacterStats {
    let hp = get_random_i32(100, 195);
    let mp = 0;
    let attack = get_random_i32(4, 7);
    let defense = get_random_i32(3, 6);
    let strength = get_random_i32(2, 4);
    let magic = 0;

    CharacterStats {
        character_class: CharacterClass::Thief,
        hp,
        mp,
        attack,
        defense,
        strength,
        magic,
    }
}

fn roll_mage_stats() -> CharacterStats {
    let hp = get_random_i32(150, 275);
    let mp = get_random_i32(15, 18);
    let attack = get_random_i32(2, 5);
    let defense = get_random_i32(3, 6);
    let strength = get_random_i32(1, 3);
    let magic = get_random_i32(4, 9);

    CharacterStats {
        character_class: CharacterClass::Mage,
        hp,
        mp,
        attack,
        defense,
        strength,
        magic,
    }
}
