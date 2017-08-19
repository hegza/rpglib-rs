use super::Display;
use std::convert::{From, Into};
use try_from::*;

#[derive(Clone)]
pub enum Equipment {
    BaseItem(BaseItem),
    ModifiedItem(ModifiedItem),
}

impl BaseItem {
    fn effects(&self) -> Vec<ItemEffect> {
        self.implicit_effects.iter().map(|x| x).cloned().collect()
    }
}

impl From<BaseItem> for Item {
    fn from(original: BaseItem) -> Self {
        Item::Equipment(original.into())
    }
}
impl From<ModifiedItem> for Item {
    fn from(original: ModifiedItem) -> Self {
        Item::Equipment(Equipment::ModifiedItem(original))
    }
}
impl From<Consumable> for Item {
    fn from(original: Consumable) -> Self {
        Item::Consumable(original)
    }
}
impl From<BaseItem> for Equipment {
    fn from(original: BaseItem) -> Self {
        Equipment::BaseItem(original)
    }
}
impl From<ModifiedItem> for Equipment {
    fn from(original: ModifiedItem) -> Self {
        Equipment::ModifiedItem(original)
    }
}
impl From<Equipment> for Item {
    fn from(original: Equipment) -> Self {
        Item::Equipment(original)
    }
}

impl Equipment {
    pub fn english_name(&self) -> String {
        match self {
            &Equipment::BaseItem(ref i) => i.english_name(),
            &Equipment::ModifiedItem(ref i) => i.english_name(),
        }
    }
    pub fn slot(&self) -> &ItemSlot {
        match self {
            &Equipment::BaseItem(ref i) => &i.slot,
            &Equipment::ModifiedItem(ref i) => &i.base.slot,
        }

    }
    pub fn effects(&self) -> Vec<ItemEffect> {
        match self {
            &Equipment::BaseItem(ref i) => i.effects(),
            &Equipment::ModifiedItem(ref i) => {
                let mut all_effects = i.base.effects();
                if let Some(ref prefix) = i.prefix {
                    let prefix_effects: Vec<ItemEffect> =
                        prefix.affix_data.effects.iter().map(|x| x).cloned().collect();
                    all_effects.extend(prefix_effects);
                }
                if let Some(ref suffix) = i.suffix {
                    let suffix_effects: Vec<ItemEffect> =
                        suffix.affix_data.effects.iter().map(|x| x).cloned().collect();
                    all_effects.extend(suffix_effects);
                }
                all_effects
            }
        }

    }
    pub fn item_quality(&self) -> ItemQuality {
        match self {
            &Equipment::BaseItem(_) => ItemQuality::Normal,
            &Equipment::ModifiedItem(_) => ItemQuality::Rare,
        }
    }
    pub fn size(&self) -> usize {
        match self {
            &Equipment::BaseItem(ref i) => i.size,
            &Equipment::ModifiedItem(ref i) => i.base.size,
        }
    }
}

#[derive(Clone)]
pub struct Consumable {
    pub size: usize,
    pub effects: Vec<ItemEffect>,
    pub english_name: String,
}

#[derive(Clone)]
pub enum Item {
    Equipment(Equipment),
    Consumable(Consumable),
}

impl Item {
    pub fn size(&self) -> usize {
        match self {
            &Item::Equipment(ref e) => {
                match e {
                    &Equipment::BaseItem(ref item) => item.size,
                    &Equipment::ModifiedItem(ref item) => item.base.size,
                }
            }
            &Item::Consumable(ref c) => c.size,
        }
    }
    pub fn english_name(&self) -> String {
        match self {
            &Item::Equipment(ref e) => {
                match e {
                    &Equipment::BaseItem(ref item) => item.english_name.clone(),
                    &Equipment::ModifiedItem(ref item) => item.english_name(),
                }
            }
            &Item::Consumable(ref c) => c.english_name.clone(),
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
    fn get<'a>(&'a self, id: i32) -> Option<&'a Item>;
    fn get_mut<'a>(&'a mut self, id: i32) -> Option<&'a mut Item>;
    fn holds_id(&self, id: usize) -> bool;

    // TODO: iter()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Attribute {
    Damage,
    MaxLife,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ItemEffect {
    AttributeModifier(Attribute, i32),
}

#[derive(Debug, Clone)]
pub struct ItemAffix {
    pub effects: Vec<ItemEffect>,
    pub english_name: String,
}

#[derive(Debug, Clone)]
pub struct ItemPrefix {
    pub affix_data: ItemAffix,
}

#[derive(Debug, Clone)]
pub struct ItemSuffix {
    pub affix_data: ItemAffix,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum ItemSlot {
    MainHand,
    OffHand,
}

impl<'a> From<&'a ItemSlot> for &'a str {
    fn from(original: &'a ItemSlot) -> &'a str {
        match original {
            &ItemSlot::MainHand => "main hand",
            &ItemSlot::OffHand => "off-hand",
        }
    }
}

impl TryFrom<Item> for Equipment {
    type Err = String;
    fn try_from(item: Item) -> Result<Equipment, Self::Err> {
        match item {
            Item::Equipment(item) => Ok(item),
            _ => Err(format!("{} can not be equipped", item.english_name())),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ItemQuality {
    Normal,
    Rare,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseItem {
    pub slot: ItemSlot,
    pub english_name: String,
    pub implicit_effects: Vec<ItemEffect>,
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct ModifiedItem {
    pub base: BaseItem,
    pub prefix: Option<ItemPrefix>,
    pub suffix: Option<ItemSuffix>,
}

impl ModifiedItem {
    pub fn with_prefix(base: BaseItem, prefix: ItemPrefix) -> ModifiedItem {
        ModifiedItem {
            base: base,
            prefix: Some(prefix),
            suffix: None,
        }
    }
    pub fn with_suffix(base: BaseItem, suffix: ItemSuffix) -> ModifiedItem {
        ModifiedItem {
            base: base,
            prefix: None,
            suffix: Some(suffix),
        }
    }
    pub fn with_affixes(base: BaseItem, prefix: ItemPrefix, suffix: ItemSuffix) -> ModifiedItem {
        ModifiedItem {
            base: base,
            prefix: Some(prefix),
            suffix: Some(suffix),
        }
    }
}

impl Display for BaseItem {
    fn english_name(&self) -> String {
        self.english_name.clone()
    }
}

impl Display for ModifiedItem {
    fn english_name(&self) -> String {
        let mut name = String::new();
        if let Some(ref prefix) = self.prefix {
            name += &prefix.affix_data.english_name;
            name += &" ";
        }
        name += &self.base.english_name;
        if let Some(ref suffix) = self.suffix {
            name += &" ";
            name += &suffix.affix_data.english_name;
        }
        name
    }
}
