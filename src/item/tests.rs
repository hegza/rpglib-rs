use item::*;
use character::*;
use item::Slot::*;
use combat::*;

#[test]
fn can_equip_items() {
        let sword = equipment("Sword", 1, &Slot::Hand, &vec![]).build();
        let shield = equipment("Shield", 1, &Slot::Hand, &vec![]).build();

        /// Arrange
        let mut character = Character::default();

        /// Act
        {
            character.equip(sword.clone().into());
            character.equip(shield.clone().into());
        }

        /// Assert
        {
            let items_in_hand = character.equipment().by_slot(&Slot::Hand);
            let main_hand = items_in_hand[0];
            let off_hand = items_in_hand[1];

            assert_eq!(items_in_hand.len(), 2);
            assert_eq!(main_hand.name(), sword.name());
            assert_eq!(off_hand.name(), shield.name());
        }
}

#[test]
fn rare_name_is_correct() {
    // Arrange
    let rare_sword = equipment("Long Sword", 1, Hand, vec![]).prefix::<Prefix>(Affix {
                                                        effects: vec![],
                                                        name: "Deadly".to_owned(),
                                                    }.into()).suffix::<Suffix>(Affix {
                                                        effects: vec![],
                                                        name: "of Slashing".to_owned(),
                                                    }.into()).build();

    // Assert
    assert_eq!(rare_sword.name(), "Deadly Long Sword of Slashing");
}

#[test]
fn sword_beats_unarmed() {
    /// Arrange
    let sword = equipment("Sword", 1, Hand, vec![]).damage(2).build();
    let mut attributes = CharacterAttributes::default();
    attributes.set(Attribute::Strength, 0);
    attributes.set(Attribute::Constitution, 5);
    let mut combatant_a = CharacterBuilder::new(2, 8, attributes.clone()).build();
    let mut combatant_b = CharacterBuilder::new(2, 8, attributes.clone()).build();

    combatant_a.equip(sword.into());

    /// Act
    {
        let mut combat = Combat::new(&combatant_a, &combatant_b);

        // Fight until either party is unable to combat
        while Combat::can_combat(&combatant_a, &combatant_b) {
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
    let item = equipment("Hardcode Sword", 1, Hand, vec![ItemEffect::AttributeModifier(Attribute::Constitution, 3)]).damage(3).build();

    /// Act
    let serialized = ::serde_yaml::to_string(&item).unwrap();
    let deserialized: Equipment = ::serde_yaml::from_str(&serialized).unwrap();

    /// Assert
    assert_eq!(item.slot, deserialized.slot);
    assert_eq!(item.name, deserialized.name);
    assert_eq!(item.effects.len(),
               deserialized.effects.len());
}

#[test]
fn put_items_in_inventory() {
    let item_1 = equipment("Long Sword", 4, Hand, vec![]).build();
    let item_2 = equipment("Short Sword", 2, Hand, vec![]).build();
    let item_3 = equipment("Short Sword", 2, Hand, vec![]).build();
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
    let item_1 = equipment("Long Sword", 4, Hand, vec![]).build();
    let item_2 = equipment("Stone", 3, Hand, vec![]).build();
    let item_3 = equipment("Short Sword", 2, Hand, vec![]).build();
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
