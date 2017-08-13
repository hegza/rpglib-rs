mod item;
mod combat;
#[macro_use]
mod utils;

#[cfg(test)]
mod tests;

pub use item::*;
pub use combat::*;
use std::collections::HashMap;

pub struct Character<'a> {
    attributes: HashMap<Attribute, i32>,
    current_life: i32,
    equipped_items: HashMap<ItemSlot, Option<&'a EquipItem>>,
}

impl<'a> Character<'a> {
    pub fn new(attributes: &HashMap<Attribute, i32>) -> Character<'a> {
        let equipped_items = hashmap![ItemSlot::MainHand => None, ItemSlot::OffHand => None];
        Character {
            attributes: attributes.clone(),
            current_life: attributes[&Attribute::MaxLife],
            equipped_items: equipped_items,
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

        // TODO: use iter
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
}
