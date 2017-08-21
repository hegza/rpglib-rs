use super::item::*;
use super::combat::*;
use std::collections::HashMap;
use super::Display;
use std::cmp::max;

pub struct Character {
    pub attributes: HashMap<Attribute, i32>,
    current_life: i32,
    equipped_items: HashMap<ItemSlot, Option<Equipment>>,
    english_name: String,
    pub inventory: Inventory,
}

impl Character {
    pub fn new(english_name: &str,
               attributes: &HashMap<Attribute, i32>,
               inventory_capacity: usize)
               -> Character {
        let equipped_items = hashmap![ItemSlot::MainHand => None, ItemSlot::OffHand => None];
        Character {
            attributes: attributes.clone(),
            current_life: attributes[&Attribute::MaxLife],
            equipped_items: equipped_items,
            english_name: english_name.to_owned(),
            inventory: Inventory::new(inventory_capacity),
        }
    }

    pub fn equip(&mut self, item: Equipment) -> Option<Equipment> {
        let mut ret = None;
        if let Some(prev) = self.equipped_items.get_mut(item.slot()) {
            ret = prev.clone();
        }
        let place = self.equipped_items.get_mut(item.slot());
        *place.unwrap() = Some(item);
        ret
    }

    pub fn unequip(&mut self, slot: &ItemSlot) -> Option<Equipment> {
        let mut ret = None;
        if let Some(item) = self.equipped_items.get_mut(slot) {
            ret = item.clone();
        }
        let place = self.equipped_items.get_mut(&slot);
        *place.unwrap() = None;
        ret
    }

    pub fn equipped_items(&self) -> &HashMap<ItemSlot, Option<Equipment>> {
        &self.equipped_items
    }
}

impl Combatant for Character {
    fn damage(&self) -> i32 {
        let from_attributes = self.attributes[&Attribute::Damage];

        let mut from_items = 0;
        for (_, item) in &self.equipped_items {
            match item {
                &None => {
                    continue;
                }
                &Some(ref item) => {
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
    fn weapon(&self) -> Option<&Equipment> {
        if let Some(ref item) = self.equipped_items[&ItemSlot::MainHand] {
            Some(item)
        } else {
            None
        }
    }
}

impl Display for Character {
    fn english_name(&self) -> String {
        self.english_name.clone()
    }
}

pub struct Inventory {
    capacity: usize,
    // Contains all items that are currently stored in this inventory.
    items: Vec<Item>,
    // Tells which positions are reserved by items. None positions are free, Some() position are reserved by items in the given index at the items vector.
    positions: Vec<Option<usize>>,
}

/// Holds items in certain positions
impl Inventory {
    pub fn new(capacity: usize) -> Inventory {
        Inventory {
            capacity: capacity,
            items: vec![],
            positions: vec![None; capacity],
        }
    }
    pub fn find_space(&self, pos: i32, size: usize) -> Option<usize> {
        if pos < 0 || pos as usize >= self.capacity() {
            return None;
        }
        if size == 0 {
            return Some(pos as usize);
        }

        let mut count_space = 0;
        for p in 0..self.capacity {
            match self.positions[p] {
                None => {
                    count_space += 1;
                }
                Some(_) => {
                    count_space = 0;
                }
            }
            if count_space == size {
                return Some(p - (size - 1));
            }
        }
        return None;
    }
    /// Moves an item into the container. Returns false if there's no room at the given position.
    pub fn put_at(&mut self, item: Item, pos: usize) -> bool {
        // Verify that all positions are free from pos to pos+size
        for i in pos..pos + item.size() {
            if self.positions[i].is_some() {
                return false;
            }
        }

        // Fill indices in the positions vector
        let idx = self.items.len();
        for i in pos..pos + item.size() {
            self.positions[i] = Some(idx);
        }
        self.items.push(item);
        true
    }
    pub fn bounds(&self, pos: i32) -> (usize, usize) {
        if pos < 0 {
            return (0, 1);
        }
        if pos >= self.capacity() as i32 {
            return (self.capacity - 1, 1);
        }
        let mut start = pos as usize;
        let mut size = 1;
        let id = self.positions[pos as usize];
        if id.is_some() {
            let mut found = false;
            for i in 0..self.capacity {
                if self.positions[i] == id {
                    // Find start
                    if !found {
                        start = i;
                        found = true;
                    } else {
                        // Start is found, but end is not
                        size += 1;
                    }
                }
            }
        }
        (start, size)
    }
    pub fn swap(&mut self, pos_1: i32, pos_2: i32) -> bool {
        // Figure out the bounds of both items
        let (start_1, size_1) = self.bounds(pos_1);
        let (start_2, size_2) = self.bounds(pos_2);

        // Check that the larger item fits in place of the smaller item
        if (size_1 > size_2 && start_2 + size_1 > self.capacity) ||
           (size_2 > size_1 && start_1 + size_2 > self.capacity) {
            return false;
        }

        // Swap all indices around
        for i in 0..max(size_1, size_2) {
            self.positions.swap(start_1 + i, start_2 + i);
        }
        true
    }
}

impl HoldsItems for Inventory {
    fn capacity(&self) -> usize {
        self.capacity
    }
    fn put(&mut self, item: Item) -> Option<usize> {
        let pos = self.find_space(0, item.size());
        match pos {
            Some(i) => {
                self.put_at(item, i);
                return Some(i);
            }
            None => None,
        }
    }
    fn take(&mut self, pos: i32) -> Option<Item> {
        if pos < 0 || pos >= self.capacity() as i32 {
            return None;
        }
        if let Some(item_idx) = self.positions[pos as usize] {

            if !self.holds_id(item_idx as usize) {
                return None;
            }

            let item = self.items.remove(item_idx);

            // Remove references to item from self.positions and decrease the id of references to items later in the vector
            for i in 0..self.capacity {
                let id_at_i = self.positions[i];
                match id_at_i {
                    None => {
                        continue;
                    }
                    Some(id) => {
                        if id > item_idx {
                            self.positions[i] = Some(id - 1);
                        }
                        if id == item_idx {
                            self.positions[i] = None;
                        }
                    }
                }
            }
            Some(item)
        } else {
            None
        }
    }
    fn get<'a>(&'a self, id: i32) -> Option<&'a Item> {
        if id < 0 || id >= self.capacity() as i32 {
            return None;
        }
        match self.positions[id as usize] {
            None => None,
            Some(i) => Some(&self.items[i]),
        }
    }
    fn get_mut<'a>(&'a mut self, id: i32) -> Option<&'a mut Item> {
        if id < 0 || id >= self.capacity() as i32 {
            return None;
        }
        match self.positions[id as usize] {
            None => None,
            Some(i) => Some(&mut self.items[i]),
        }
    }
    fn holds_id(&self, id: usize) -> bool {
        self.items.len() > id
    }
}
