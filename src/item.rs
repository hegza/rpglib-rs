#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Attribute {
    Damage,
    MaxLife,
}

#[derive(Clone)]
pub enum ItemEffect {
    AttributeModifier(Attribute, i32),
}

pub struct ItemAffix {
    pub effects: Vec<ItemEffect>,
    pub english_name: String,
}

pub struct ItemPrefix {
    pub affix_data: ItemAffix,
}

pub struct ItemSuffix {
    pub affix_data: ItemAffix,
}

#[derive(PartialEq, Eq, Hash)]
pub enum ItemSlot {
    MainHand,
    OffHand,
}

pub trait EquipItem {
    fn slot(&self) -> &ItemSlot;
    fn effects(&self) -> Vec<ItemEffect>;
    fn english_name(&self) -> String;
}

pub enum ItemQuality {
    Normal,
    Rare,
}

pub struct BaseItem {
    pub slot: ItemSlot,
    pub quality: ItemQuality,
    pub english_name: String,
    pub implicit_effects: Vec<ItemEffect>,
}

impl EquipItem for BaseItem {
    fn slot(&self) -> &ItemSlot {
        &self.slot
    }
    fn effects(&self) -> Vec<ItemEffect> {
        self.implicit_effects.iter().map(|x| x).cloned().collect()

    }
    fn english_name(&self) -> String {
        self.english_name.clone()
    }
}

pub struct RareItem {
    pub base: BaseItem,
    pub prefix: ItemPrefix,
    pub suffix: ItemSuffix,
}

impl EquipItem for RareItem {
    fn slot(&self) -> &ItemSlot {
        &self.base.slot
    }
    fn effects(&self) -> Vec<ItemEffect> {
        let mut all_effects = self.base.effects();
        let prefix_effects: Vec<ItemEffect> =
            self.prefix.affix_data.effects.iter().map(|x| x).cloned().collect();
        let suffix_effects: Vec<ItemEffect> =
            self.suffix.affix_data.effects.iter().map(|x| x).cloned().collect();
        all_effects.extend(prefix_effects);
        all_effects.extend(suffix_effects);
        all_effects
    }
    fn english_name(&self) -> String {
        format!("{} {} {}",
                self.prefix.affix_data.english_name,
                self.base.english_name,
                self.suffix.affix_data.english_name)
    }
}
