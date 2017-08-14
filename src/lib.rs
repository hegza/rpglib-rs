extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

mod item;
mod combat;
mod reward;
#[macro_use]
mod utils;

#[cfg(test)]
mod tests;

pub use item::*;
pub use combat::*;
pub use reward::*;
use std::collections::HashMap;

pub struct Character<'a> {
    attributes: HashMap<Attribute, i32>,
    current_life: i32,
    equipped_items: HashMap<ItemSlot, Option<&'a EquipItem>>,
    english_name: String,
}

impl<'a> Character<'a> {
    pub fn new(english_name: &str, attributes: &HashMap<Attribute, i32>) -> Character<'a> {
        let equipped_items = hashmap![ItemSlot::MainHand => None, ItemSlot::OffHand => None];
        Character {
            attributes: attributes.clone(),
            current_life: attributes[&Attribute::MaxLife],
            equipped_items: equipped_items,
            english_name: english_name.to_owned(),
        }
    }

    pub fn equip(&mut self, item: &'a EquipItem) {
        let slot = item.slot();
        *self.equipped_items.get_mut(slot).unwrap() = Some(item);
    }

    pub fn equipped_items(&self) -> &HashMap<ItemSlot, Option<&'a EquipItem>> {
        &self.equipped_items
    }
}

impl<'a> Combatant for Character<'a> {
    fn damage(&self) -> i32 {
        let from_attributes = self.attributes[&Attribute::Damage];

        let mut from_items = 0;
        for (_, item) in &self.equipped_items {
            if item.is_some() {
                let item = item.unwrap();
                for effect in item.effects() {
                    match effect {
                        ItemEffect::AttributeModifier(attr, amount) => {
                            if attr == Attribute::Damage {
                                from_items += amount;
                            }
                        }
                    }
                }
            }
        }

        from_attributes + from_items
    }
    fn reduce_life(&mut self, amount: i32) -> i32 {
        self.current_life -= amount;
        if self.current_life < 0 {
            self.current_life = 0;
        }
        self.current_life
    }
    fn life(&self) -> i32 {
        self.current_life
    }
    fn can_combat(&self) -> bool {
        self.current_life > 0
    }
    fn english_name(&self) -> String {
        self.english_name.clone()
    }
}

pub struct Monster<'a> {
    max_life: i32,
    current_life: i32,
    damage: i32,
    english_name: String,
    reward: &'a Reward<'a>,
}

impl<'a> Monster<'a> {
    pub fn new(english_name: &str, damage: i32, life: i32, reward: &'a Reward) -> Monster<'a> {
        Monster {
            max_life: life,
            current_life: life,
            damage: damage,
            english_name: english_name.to_owned(),
            reward: reward,
        }
    }
}

impl<'a> Combatant for Monster<'a> {
    fn damage(&self) -> i32 {
        self.damage
    }
    fn reduce_life(&mut self, amount: i32) -> i32 {
        self.current_life -= amount;
        if self.current_life < 0 {
            self.current_life = 0;
        }
        self.current_life
    }
    fn life(&self) -> i32 {
        self.current_life
    }
    fn can_combat(&self) -> bool {
        self.current_life > 0
    }
    fn english_name(&self) -> String {
        self.english_name.clone()
    }
}

impl<'b> YieldReward<'b> for Monster<'b> {
    fn reward<'a>(&'a self) -> &'a Reward<'b> {
        &self.reward
    }
}
