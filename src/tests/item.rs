use super::super::*;
use super::super::combat::*;
use super::super::item::*;

#[test]
fn can_equip_items() {
    {
        let sword = BaseItem {
            slot: ItemSlot::MainHand,
            english_name: "Sword".to_owned(),
            implicit_effects: vec![],
            size: 1,
        };
        let shield = BaseItem {
            slot: ItemSlot::OffHand,
            english_name: "Shield".to_owned(),
            implicit_effects: vec![],
            size: 1,
        };

        /// Arrange
        let attributes = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
        let mut character = Character::new("", &attributes, 8);

        /// Act
        {
            character.equip(sword.clone().into());
            character.equip(shield.clone().into());
        }

        /// Assert
        {
            let main_hand = match character.equipped_items()[&ItemSlot::MainHand] {
                None => None,
                Some(ref item_box) => Some(item_box),
            };
            let off_hand = match character.equipped_items()[&ItemSlot::OffHand] {
                None => None,
                Some(ref item_box) => Some(item_box),
            };
            assert_eq!(character.equipped_items().len(), 2);
            assert_eq!(main_hand.unwrap().english_name(), sword.english_name);
            assert_eq!(off_hand.unwrap().english_name(), shield.english_name);
        }
    }
}

#[test]
fn rare_name_is_correct() {
    // Arrange
    let rare_sword = ModifiedItem::with_affixes(BaseItem {
                                                    slot: ItemSlot::MainHand,
                                                    english_name: "Long Sword".to_owned(),
                                                    implicit_effects: vec![],
                                                    size: 1,
                                                },
                                                ItemPrefix {
                                                    affix_data: ItemAffix {
                                                        effects: vec![],
                                                        english_name: "Deadly".to_owned(),
                                                    },
                                                },
                                                ItemSuffix {
                                                    affix_data: ItemAffix {
                                                        effects: vec![],
                                                        english_name: "of Slashing".to_owned(),
                                                    },
                                                });

    // Assert
    assert_eq!(rare_sword.english_name(), "Deadly Long Sword of Slashing");
}
#[test]
fn sword_beats_unarmed() {
    /// Arrange
    let attributes = hashmap![Attribute::MaxLife => 8, Attribute::Damage => 1];
    let sword = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Sword".to_owned(),
        implicit_effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, 2)],
        size: 1,
    };
    let mut combatant_a = Character::new("", &attributes, 8);
    let mut combatant_b = Character::new("", &attributes, 8);

    combatant_a.equip(sword.into());

    /// Act
    {
        let mut combat = Combat::new();

        // Fight until either party is unable to combat
        while combat.can_combat(&combatant_a, &combatant_b) {
            combat.apply_round(&mut combatant_a, &mut combatant_b);
        }
    }

    /// Assert
    assert!(combatant_a.life() != 0);
    assert_eq!(combatant_b.life(), 0);
}

#[test]
fn item_can_serde() {
    /// Arrange
    let item = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Hardcode Sword".to_owned(),
        implicit_effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, 3),
                               ItemEffect::AttributeModifier(Attribute::MaxLife, 3)],
        size: 1,
    };

    /// Act
    let serialized = serde_yaml::to_string(&item).unwrap();
    let deserialized: BaseItem = serde_yaml::from_str(&serialized).unwrap();

    /// Assert
    assert_eq!(item.slot, deserialized.slot);
    assert_eq!(item.english_name, deserialized.english_name);
    assert_eq!(item.implicit_effects.len(),
               deserialized.implicit_effects.len());
}

#[test]
fn put_items_in_inventory() {
    let item_1 = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Long Sword".to_owned(),
        implicit_effects: vec![],
        size: 4,
    };
    let item_2 = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Short Sword".to_owned(),
        implicit_effects: vec![],
        size: 2,
    };
    let item_3 = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Short Sword".to_owned(),
        implicit_effects: vec![],
        size: 2,
    };
    let mut inventory = Inventory::new(8);

    /// Act
    let pos_1 = inventory.put(item_1.into());
    let pos_2 = inventory.put(item_2.into());
    let pos_3 = inventory.put(item_3.into());

    /// Assert
    assert!(pos_1.is_some());
    assert!(pos_2.is_some());
    assert!(pos_3.is_some());
    assert_eq!(pos_1.unwrap(), 0);
    assert_eq!(pos_2.unwrap(), 4);
    assert_eq!(pos_3.unwrap(), 6);
}

#[test]
fn complex_inventory() {
    let item_1 = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Long Sword".to_owned(),
        implicit_effects: vec![],
        size: 4,
    };
    let item_2 = BaseItem {
        slot: ItemSlot::OffHand,
        english_name: "Stone".to_owned(),
        implicit_effects: vec![],
        size: 3,
    };
    let item_3 = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Short Sword".to_owned(),
        implicit_effects: vec![],
        size: 2,
    };
    let mut inventory = Inventory::new(9);

    /// Act
    let pos_1 = inventory.put(item_1.into());
    let pos_2 = inventory.put(item_2.into());
    let pos_3 = inventory.put(item_3.into());
    inventory.take(pos_2.unwrap() as i32);

    /// Assert
    assert_eq!(pos_1.unwrap(), 0);
    assert!(inventory.take(4).is_none());
    assert!(inventory.take(5).is_none());
    assert!(inventory.take(6).is_none());
    assert_eq!(pos_3.unwrap(), 7);
}
