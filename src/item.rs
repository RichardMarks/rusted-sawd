use crate::character::CharacterClass;

#[derive(Debug, Default, Clone)]
pub struct StatModifier {
    pub text: String,
    pub value: i32,
}

#[derive(Debug, Default, Clone)]
pub struct GameItemStatModifiers {
    pub max_hp: StatModifier,
    pub max_mp: StatModifier,
    pub max_ap: StatModifier,
    pub attack: StatModifier,
    pub defense: StatModifier,
    pub strength: StatModifier,
    pub magic: StatModifier,
    pub hp: StatModifier,
    pub mp: StatModifier,
    pub ap: StatModifier,
}

#[derive(Debug, Default, Clone)]
pub enum GameItemType {
    #[default]
    Consumable,
    Equipment,
}

#[derive(Debug, Default)]
pub struct GameItem {
    pub name: String,
    pub item_type: GameItemType,
    pub character_class: CharacterClass,
    pub modifiers: GameItemStatModifiers,
}

impl Clone for GameItem {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            item_type: self.item_type.clone(),
            character_class: self.character_class.clone(),
            modifiers: self.modifiers.clone(),
        }
    }
}
