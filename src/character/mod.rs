mod attribute;
mod inventory;
mod builder;

pub use self::attribute::*;
pub use self::inventory::*;
pub use self::builder::*;

use super::item::*;
use super::combat::*;
use super::Display;
use std::cmp::max;

#[derive(Clone)]
pub struct Character {
    base_attributes: CharacterAttributes,
    current_life: i32,
    equipment: EquipmentStore,
    name: String,
    action_buffer: ActionBuffer,
    pub inventory: Inventory,
}

impl Character {
    pub fn slots(&self) -> Vec<&Slot> {
        self.equipment.slots()
    }
    pub fn equipment(&self) -> &EquipmentStore {
        &self.equipment
    }
    pub fn equip(&mut self, item: Equipment) -> Option<Equipment> {
        self.equipment.equip(item)
    }
    pub fn unequip(&mut self, slot: &Slot) -> Option<Equipment> {
        self.equipment.unequip(slot)
    }
    pub fn attribute(&self, attr: &Attribute) -> i32 {
        // Innate ability of character
        let base = self.base_attributes.get(attr);

        // Combined bonuses from worn items
        let mut from_items = 0;
        for &(_, ref item) in &self.equipment.items {
            match *item {
                None => {
                    continue;
                }
                Some(ref item) => {
                    for effect in item.effects() {
                        match effect {
                            ItemEffect::AttributeModifier(ref id, amount) => {
                                if id == attr {
                                    from_items += amount;
                                }
                            }
                        }
                    }
                }
            }
        }

        base + from_items
    }
    pub fn nth_slot(&self, n: usize) -> Option<&Slot> {
        self.equipment.nth_slot(n)
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            base_attributes: CharacterAttributes::default(),
            current_life: 1,
            equipment: EquipmentStore {
                items: vec![
                    (Slot::Head, None),
                    (Slot::Hand, None),
                    (Slot::Torso, None),
                    (Slot::Hand, None),
                    (Slot::Feet, None),
                ]
            },
            name: String::new(),
            action_buffer: ActionBuffer::default(),
            inventory: Inventory::new(8),
        }
    }
}

#[derive(Clone)]
pub struct EquipmentStore {
    items: Vec<(Slot, Option<Equipment>)>,
}

impl EquipmentStore {
    pub fn equip(&mut self, item: Equipment) -> Option<Equipment> {
        let first_free = self.first_free(item.slot());
        match first_free {
            // Available slot: equip in that slot
            Some(idx) => {
                let (_, ref mut slot) = self.items[idx];
                *slot = Some(item);
                None
            }
            // No available slot: unequip something
            None => {
                let first = self.first(item.slot());
                match first {
                    Some(idx) => {
                        let (_, ref mut slot) = self.items[idx];
                        let prev_equip = slot.clone();
                        *slot = Some(item);
                        prev_equip
                    }
                    None => None,
                }
            }
        }
    }
    pub fn unequip(&mut self, slot: &Slot) -> Option<Equipment> {
        let first_reserved = self.first_reserved(slot);
        match first_reserved {
            Some(idx) => {
                let (_, ref mut slot) = self.items[idx];
                let prev_equip = slot.clone();
                *slot = None;
                prev_equip
            }
            None => None,
        }
    }
    fn first_free(&self, slot: &Slot) -> Option<usize> {
        self.items.iter().position(|&(ref s, ref option)| s == slot && option.is_none())
    }
    fn first(&self, slot: &Slot) -> Option<usize> {
        self.items.iter().position(|&(ref s, _)| s == slot)
    }
    fn first_reserved(&self, slot: &Slot) -> Option<usize> {
        self.items.iter().position(|&(ref s, ref option)| s == slot && option.is_some())
    }
    pub fn first_in_slot(&self, slot: &Slot) -> Option<&Equipment> {
        let first = self.first_reserved(slot);
        match first {
            None => None,
            Some(idx) => {
                // items: Vec<(Slot, Option<Equipment>)>
                let (_, ref slot) = self.items[idx];
                match *slot {
                    Some(ref equipment) => Some(equipment),
                    None => unreachable!(),
                }
            }
        }
    }
    pub fn nth_slot(&self, n: usize) -> Option<&Slot> {
        let entry = self.items.iter().nth(n);
        match entry {
            None => None,
            Some( &(ref slot, _) ) => Some(slot),
        }
    }
    pub fn nth_in_slot(&self, slot: &Slot, n: usize) -> Option<&Equipment> {
        let slot_option = self.items.iter().filter(|&&(ref s, _)| s == slot).nth(n);
        match slot_option {
            None => None,
            Some(&(_, ref o_item)) => match *o_item {
                None => None,
                Some(ref item) => Some(item),
            },
        }
    }
    pub fn by_slot(&self, slot: &Slot) -> Vec<&Equipment> {
        self.items.iter()
            // Filter to slots of correct type
            .filter(|&&(ref s, _)| s == slot)
            // Filter to slots containing some
            .filter(|&&(_, ref item)| item.is_some())
            // TODO: can be made more concise by using Option<T>.as_ref() -> Option<&T>s
            .map(|&(_, ref item_option)| match *item_option {
                None => None,
                Some(ref item) => Some(item),
            }.unwrap())
            // Return references
            .collect()
    }
    pub fn inner_mut(&mut self) -> &mut Vec<(Slot, Option<Equipment>)> {
        &mut self.items
    }
    pub fn inner(&self) -> &Vec<(Slot, Option<Equipment>)> {
        &self.items
    }
    pub fn slots(&self) -> Vec<&Slot> {
        self.items.iter().map(|&(ref slot, _)| slot).collect()
    }
}

impl Default for EquipmentStore {
    fn default() -> Self {
        EquipmentStore {
            items: vec![
                (Slot::Head, None),
                (Slot::Hand, None),
                (Slot::Torso, None),
                (Slot::Hand, None),
                (Slot::Feet, None),
            ]}
    }
}

impl Combatant for Character {
    fn damage(&self) -> i32 {
        // Strength is added to the damage value of any weapon
        let strength = self.attribute(&Attribute::Strength);

        // Check damage on each item in hand
        let items_in_hands = self.equipment.by_slot(&Slot::Hand);

        // If empty handed, hit with strength only
        if items_in_hands.len() == 0 {
            return strength;
        }
        let highest_damage = items_in_hands.iter().map(|&item| item.damage() + strength).max();

        highest_damage.unwrap()
    }
    fn action_buffer(&self) -> ActionBuffer {
        ActionBuffer::default()
    }
    fn set_life(&mut self, amount: i32) -> i32 {
        self.current_life = max(amount, 0);
        self.current_life
    }
    fn life(&self) -> i32 {
        self.current_life
    }
    fn can_combat(&self) -> bool {
        self.current_life > 0
    }
}

impl Display for Character {
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone)]
pub struct CharacterAttributes {
    strength: i32,
    constitution: i32,
    endurance: i32,
    swiftness: i32,
}

impl Default for CharacterAttributes {
    fn default() -> Self {
        Self {strength: 1, constitution: 1, endurance: 1, swiftness: 1}
    }
}

impl CharacterAttributes {
    pub fn get<A: AsRef<Attribute>>(&self, attribute: A) -> i32 {
        use Attribute::*;
        match *attribute.as_ref() {
            Strength => self.strength,
            Constitution => self.constitution,
            Endurance => self.endurance,
            Swiftness => self.swiftness,
        }
    }
    pub fn get_mut<A: AsRef<Attribute>>(&mut self, attribute: A) -> &mut i32 {
        use Attribute::*;
        match *attribute.as_ref() {
            Strength => &mut self.strength,
            Constitution => &mut self.constitution,
            Endurance => &mut self.endurance,
            Swiftness => &mut self.swiftness,
        }
    }
    pub fn set<A: AsRef<Attribute>>(&mut self, attribute: A, val: i32) {
        *self.get_mut(attribute) = val;
    }
}

impl AsRef<CharacterAttributes> for CharacterAttributes {
    fn as_ref(&self) -> &Self {
        self
    }
}
