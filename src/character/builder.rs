use super::*;

impl CharacterBuilder {
    pub fn new<CA: AsRef<CharacterAttributes>>(
        max_actions: usize,
        inventory_space: usize,
        attributes: CA,
    ) -> Self {
        let attributes = attributes.as_ref();
        let life = attributes.get(&Attribute::Constitution);
        Self {
            character: Character {
                base_attributes: attributes.clone(),
                current_life: life,
                equipment: EquipmentStore::default(),
                name: String::new(),
                action_buffer: ActionBuffer::new(max_actions),
                inventory: Inventory::new(inventory_space),
                available_actions: vec![Action::Attack]
            },
        }
    }
    pub fn named(mut self, name: &str) -> Self {
        self.character.name = name.to_owned();
        self
    }
    pub fn add_slot<S: AsRef<Slot>>(mut self, slot: S) -> Self {
        self.character
            .equipment
            .inner_mut()
            .push((slot.as_ref().clone(), None));
        self
    }
    pub fn build(&self) -> Character {
        self.character.clone()
    }
}

pub struct CharacterBuilder {
    character: Character,
}
