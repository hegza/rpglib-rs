mod affix;
mod builder;
#[cfg(test)]
mod tests;

pub use self::builder::*;
pub use self::affix::*;

use super::{Display, DisplayWeapon};
use std::convert::{From, Into};
use try_from::*;
use character::Attribute;

#[derive(Clone)]
pub enum Item {
    Consumable(Consumable),
    Equipment(Equipment),
}

#[derive(Clone)]
pub struct Consumable {
    /// Amount of space taken while in an inventory.
    size: usize,
    /// Implicit effects of the consumable type.
    effects: Vec<ItemEffect>,
    name: String,
    max_uses: usize,
    uses: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equipment {
    slot: Slot,
    name: String,
    /// Implicit effects of the equipment type.
    effects: Vec<ItemEffect>,
    size: usize,
    /// Damage when used to hit something, should likely be 0 for things that are not used to hit something.
    damage: i32,
    prefix: Option<Prefix>,
    suffix: Option<Suffix>,
}

impl Display for Equipment {
    fn name(&self) -> String {
        let mut name = String::new();
        if let Some(ref prefix) = self.prefix {
            name += &prefix.affix_data.name;
            name += &" ";
        }
        name += &self.name;
        if let Some(ref suffix) = self.suffix {
            name += &" ";
            name += &suffix.affix_data.name;
        }
        name
    }
}

impl DisplayWeapon for Equipment {}

impl Equipment {
    pub fn slot(&self) -> &Slot {
        &self.slot
    }
    pub fn effects(&self) -> Vec<ItemEffect> {
        let mut all_effects = self.effects.clone();
        if let Some(ref prefix) = self.prefix {
            let prefix_effects: Vec<ItemEffect> =
                prefix.affix_data.effects.iter().map(|x| x).cloned().collect();
            all_effects.extend(prefix_effects);
        }
        if let Some(ref suffix) = self.suffix {
            let suffix_effects: Vec<ItemEffect> =
                suffix.affix_data.effects.iter().map(|x| x).cloned().collect();
            all_effects.extend(suffix_effects);
        }
        all_effects
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn damage(&self) -> i32 {
        self.damage
    }
}

impl Display for Consumable {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Item {
    pub fn size(&self) -> usize {
        match *self {
            Item::Equipment(ref e) => {
                e.size()
            },
            Item::Consumable(ref c) => c.size,
        }
    }
}

impl Display for Item {
    fn name(&self) -> String {
        match *self {
            Item::Equipment(ref e) =>
                e.name(),
            Item::Consumable(ref c) => c.name.clone(),
        }
    }
}

/// Something that can hold items. Makes no guarantees about how items are stored.
pub trait HoldsItems {
    /// Number of size-units that can fit into the container in total.
    fn capacity(&self) -> usize;
    /// Moves an item into the container. Returns an identifier that can be used to get the item. Returns None if there's no room.
    fn put(&mut self, item: Item) -> Option<usize>;
    /// Takes an item from the container
    fn take(&mut self, id: i32) -> Option<Item>;
    /// Returns an item in pos for reading. This position does not have to be the starting position of the item.
    fn get(&self, id: i32) -> Option<&Item>;
    fn get_mut(&mut self, id: i32) -> Option<&mut Item>;
    fn get_clone(&self, pos: i32) -> Option<Item>;
    fn holds_id(&self, id: usize) -> bool;

    // TODO: iter()
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ItemEffect {
    AttributeModifier(Attribute, i32),
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum Slot {
    Head,
    Hand,
    Torso,
    Feet,
}

impl<'a> From<&'a Slot> for &'a str {
    fn from(original: &'a Slot) -> &'a str {
        use Slot::*;
        match *original {
            Head => "head",
            Hand => "hand",
            Torso => "torso",
            Feet => "feet",
        }
    }
}

impl From<Equipment> for Item {
    fn from(original: Equipment) -> Self {
        Item::Equipment(original.into())
    }
}

impl TryFrom<Item> for Equipment {
    type Err = String;
    fn try_from(original: Item) -> Result<Self, Self::Err> {
        match original {
            Item::Equipment(e) => {
                Ok(e)
            },
            _ => Err(format!("cannot convert item to equipment: {}", &original.name())),
        }
    }
}

impl From<Consumable> for Item {
    fn from(original: Consumable) -> Self {
        Item::Consumable(original.into())
    }
}

impl TryFrom<Item> for Consumable {
    type Err = String;
    fn try_from(original: Item) -> Result<Self, Self::Err> {
        match original {
            Item::Consumable(e) => {
                Ok(e)
            },
            _ => Err(format!("cannot convert item to consumable: {}", &original.name())),
        }
    }
}

impl AsRef<Slot> for Slot {
    fn as_ref(&self) -> &Self {
        self
    }
}
