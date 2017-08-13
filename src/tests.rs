use super::*;
use super::combat::*;
use super::item::*;

#[test]
fn can_equip_items() {
    {
        let sword = BaseItem {
            slot: ItemSlot::MainHand,
            quality: ItemQuality::Normal,
            english_name: "Sword".to_owned(),
            implicit_effects: vec![],
        };
        let shield = BaseItem {
            slot: ItemSlot::OffHand,
            quality: ItemQuality::Normal,
            english_name: "Shield".to_owned(),
            implicit_effects: vec![],
        };

        /// Arrange
        let attributes = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
        let mut character = Character::new(&attributes);

        /// Act
        {
            character.equip(&sword);
            character.equip(&shield);
        }

        /// Assert
        {
            assert_eq!(character.equipped_items().len(), 2);
            assert_eq!(character.equipped_items()[&ItemSlot::MainHand].unwrap().english_name(),
                       sword.english_name);
            assert_eq!(character.equipped_items()[&ItemSlot::OffHand].unwrap().english_name(),
                       shield.english_name);
        }
    }
}

#[test]
fn rare_name_is_correct() {
    // Arrange
    let rare_sword = RareItem {
        base: BaseItem {
            slot: ItemSlot::MainHand,
            quality: ItemQuality::Rare,
            english_name: "Long Sword".to_owned(),
            implicit_effects: vec![],
        },
        prefix: ItemPrefix {
            affix_data: ItemAffix {
                effects: vec![],
                english_name: "Deadly".to_owned(),
            },
        },
        suffix: ItemSuffix {
            affix_data: ItemAffix {
                effects: vec![],
                english_name: "of Slashing".to_owned(),
            },
        },
    };

    // Assert
    assert_eq!(rare_sword.english_name(), "Deadly Long Sword of Slashing");
}

#[test]
fn combat_works() {
    /// Arrange
    let attributes = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
    // TODO: use trait for Combatant
    let mut combatant_a = Character::new(&attributes);
    let mut combatant_b = Character::new(&attributes);

    /// Act
    let mut combat_duration = 0;
    {
        let mut combat = Combat::new(&mut combatant_a, &mut combatant_b);

        // Fight until either party is unable to combat
        while combat.can_combat() {
            combat.apply_round();
        }

        let end_results = combat.end_combat();
        combat_duration = end_results.combat_duration;
    }

    /// Assert
    assert_eq!(combatant_a.current_life, 0);
    assert_eq!(combatant_b.current_life, 0);
    assert_eq!(combat_duration, 3);
}

#[test]
fn sword_beats_unarmed() {
    /// Arrange
    let attributes = hashmap![Attribute::MaxLife => 8, Attribute::Damage => 1];
    let sword = BaseItem {
        slot: ItemSlot::MainHand,
        quality: ItemQuality::Normal,
        english_name: "Sword".to_owned(),
        implicit_effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, 2)],
    };
    // TODO: use trait for Combatant
    let mut combatant_a = Character::new(&attributes);
    let mut combatant_b = Character::new(&attributes);

    combatant_a.equip(&sword);

    /// Act
    {
        let mut combat = Combat::new(&mut combatant_a, &mut combatant_b);

        // Fight until either party is unable to combat
        while combat.can_combat() {
            combat.apply_round();
        }

        combat.end_combat();
    }

    /// Assert
    assert!(combatant_a.current_life != 0);
    assert_eq!(combatant_b.current_life, 0);
}

//#[test]
// TODO: test item deserialization

//#[test]
// TODO: test item serialization
