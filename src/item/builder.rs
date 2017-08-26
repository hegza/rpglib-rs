use super::*;

pub fn consumable<IEVec: AsRef<Vec<ItemEffect>>>(name: &str, size: usize, effects: IEVec) -> ConsumableBuilder {
    ConsumableBuilder {
        consumable: Consumable {
            size,
            effects: effects.as_ref().clone(),
            name: name.to_owned(),
            max_uses: 1,
            uses: 1,
        }
    }
}

pub fn equipment<S: AsRef<Slot>, IEVec: AsRef<Vec<ItemEffect>>>(name: &str, size: usize, slot: S, effects: IEVec) -> EquipmentBuilder {
    EquipmentBuilder {
        equipment: Equipment {
            slot: slot.as_ref().clone(),
            name: name.to_owned(),
            effects: effects.as_ref().clone(),
            size: size,
            damage: 0,
            prefix: None,
            suffix: None,
        }
    }
}

pub struct ConsumableBuilder {
    consumable: Consumable
}

pub struct EquipmentBuilder {
    equipment: Equipment
}

impl ConsumableBuilder {
    pub fn build(&self) -> Consumable {
        self.consumable.clone()
    }
    pub fn uses(mut self, uses: usize) -> ConsumableBuilder {
        self.consumable.uses = uses;
        self.consumable.max_uses = uses;
        self
    }
    pub fn remaining_uses(mut self, uses: usize, max_uses: usize) -> ConsumableBuilder {
        self.consumable.uses = uses;
        self.consumable.max_uses = max_uses;
        self
    }
}

impl EquipmentBuilder {
    pub fn build(&self) -> Equipment {
        self.equipment.clone()
    }
    pub fn damage(mut self, damage: i32) -> EquipmentBuilder {
        self.equipment.damage = damage;
        self
    }
    pub fn prefix<P: AsRef<Prefix>>(mut self, prefix: P) -> EquipmentBuilder {
        self.equipment.prefix = Some(prefix.as_ref().clone());
        self
    }
    pub fn suffix<S: AsRef<Suffix>>(mut self, suffix: S) -> EquipmentBuilder {
        self.equipment.suffix = Some(suffix.as_ref().clone());
        self
    }
}
