use super::*;

#[test]
fn combat_works() {
    // Arrange
    let mut combatant_a = Character::default();
    let mut combatant_b = Character::default();

    // Act
    let duration = {
        let mut combat = Combat::new(&combatant_a, &combatant_b);

        // Fight until either party is unable to combat
        while Combat::can_combat(&combatant_a, &combatant_b) {
            combat.apply_round(&mut combatant_a, &mut combatant_b);
        }

        combat.duration
    };

    // Assert
    assert_eq!(combatant_a.life(), 0);
    assert_eq!(combatant_b.life(), 0);
    assert_eq!(duration, 1);
}

#[test]
fn winner_is_declared() {
    // Arrange
    use Attribute::*;
    let mut attributes_a = CharacterAttributes::default();
    attributes_a.set(Constitution, 3);
    attributes_a.set(Strength, 1);
    let mut attributes_b = CharacterAttributes::default();
    attributes_b.set(Constitution, 5);
    attributes_b.set(Strength, 1);
    let mut combatant_a = CharacterBuilder::new(2, 8, &attributes_a).build();
    let mut combatant_b = CharacterBuilder::new(2, 8, &attributes_b).build();

    // Act
    let winner_str;
    {
        let mut combat = Combat::new(&combatant_a, &combatant_b);

        // Fight until either party is unable to combat
        while Combat::can_combat(&combatant_a, &combatant_b) {
            combat.apply_round(&mut combatant_a, &mut combatant_b);
        }

        if let Results::End { winner, .. } = combat.results {
            let winner = winner.to_combatant(&combatant_a, &combatant_b);
            winner_str = winner.name();
        } else {
            unreachable!();
        }
    };

    // Assert
    assert_eq!(winner_str, combatant_b.name());
}

#[test]
fn monster_can_be_built() {
    MonsterBuilder::new("name", 1, 3).difficulty(1).spawn();
}

#[test]
fn no_duplicate_uids() {
    /*
    let monster = MonsterBuilder::new("m", 1, 2).difficulty(3).spawn();
    let monster2 = MonsterBuilder::new("m2", 1, 2).difficulty(3).spawn();
    let character = Character::new("c", hashmap![], 1);
    let character2 = Character::new("c2", hashmap![], 1);
    let equip = Item::Equipment(Equipment::BaseItem(BaseItem {}));
    let equip2 = Item::Equipment(Equipment::BaseItem(BaseItem {}));
    let consumable = Item::Consumable::new();
    */
}
