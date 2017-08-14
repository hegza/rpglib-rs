use super::*;
use super::combat::*;
use super::item::*;

#[test]
fn can_equip_items() {
    {
        let sword = BaseItem {
            slot: ItemSlot::MainHand,
            english_name: "Sword".to_owned(),
            implicit_effects: vec![],
        };
        let shield = BaseItem {
            slot: ItemSlot::OffHand,
            english_name: "Shield".to_owned(),
            implicit_effects: vec![],
        };

        /// Arrange
        let attributes = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
        let mut character = Character::new("", &attributes);

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
    let mut combatant_a = Character::new("", &attributes);
    let mut combatant_b = Character::new("", &attributes);

    /// Act
    let combat_duration;
    {
        let mut combat = Combat::new(&mut combatant_a, &mut combatant_b);

        // Fight until either party is unable to combat
        while combat.can_combat() {
            combat.apply_round();
        }

        let end_results = combat.end_results();
        combat_duration = end_results.combat_duration;
    }

    /// Assert
    assert_eq!(combatant_a.current_life, 0);
    assert_eq!(combatant_b.current_life, 0);
    assert_eq!(combat_duration, 3);
}

#[test]
fn winner_is_declared() {
    /// Arrange
    let attributes_a = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
    let attributes_b = hashmap![Attribute::MaxLife => 5, Attribute::Damage => 1];
    let mut combatant_a = Character::new("A", &attributes_a);
    let mut combatant_b = Character::new("B", &attributes_b);

    /// Act
    let winner = {
        let mut combat = Combat::new(&mut combatant_a, &mut combatant_b);

        // Fight until either party is unable to combat
        while combat.can_combat() {
            // Assert: there must not be a winner while the combatants can combat
            assert!(combat.winner().is_none());

            combat.apply_round();
        }

        combat.winner().unwrap().english_name()
    };

    /// Assert
    assert_eq!(winner, combatant_b.english_name());
}

#[test]
fn sword_beats_unarmed() {
    /// Arrange
    let attributes = hashmap![Attribute::MaxLife => 8, Attribute::Damage => 1];
    let sword = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Sword".to_owned(),
        implicit_effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, 2)],
    };
    let mut combatant_a = Character::new("", &attributes);
    let mut combatant_b = Character::new("", &attributes);

    combatant_a.equip(&sword);

    /// Act
    {
        let mut combat = Combat::new(&mut combatant_a, &mut combatant_b);

        // Fight until either party is unable to combat
        while combat.can_combat() {
            combat.apply_round();
        }
    }

    /// Assert
    assert!(combatant_a.current_life != 0);
    assert_eq!(combatant_b.current_life, 0);
}

#[test]
fn item_can_serde() {
    /// Arrange
    let item = BaseItem {
        slot: ItemSlot::MainHand,
        english_name: "Hardcode Sword".to_owned(),
        implicit_effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, 3),
                               ItemEffect::AttributeModifier(Attribute::MaxLife, 3)],
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
fn reward_is_usable() {
    let damages = vec![1, 2, 4, 8];
    let item = RareItem {
        base: BaseItem {
            slot: ItemSlot::MainHand,
            english_name: "Long Sword".to_owned(),
            implicit_effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, damages[1])],
        },
        prefix: ItemPrefix {
            affix_data: ItemAffix {
                effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, damages[2])],
                english_name: "Deadly".to_owned(),
            },
        },
        suffix: ItemSuffix {
            affix_data: ItemAffix {
                effects: vec![ItemEffect::AttributeModifier(Attribute::Damage, damages[3])],
                english_name: "of Slashing".to_owned(),
            },
        },
    };
    let reward_item = Reward::Item(&item);
    let enemy = Monster::new("Enemy", 1, 3, &reward_item);
    let attributes = hashmap![Attribute::MaxLife => 5, Attribute::Damage => damages[0]];
    let mut player = Character::new("Player", &attributes);

    let reward = enemy.reward();
    match reward {
        &Reward::Item(item) => {
            player.equip(item);
        }
    }

    /// Assert
    assert_eq!(player.damage(), damages.iter().sum::<i32>());
}