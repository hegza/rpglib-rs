mod item;
mod dungeon;

use super::*;

#[test]
fn combat_works() {
    // Arrange
    let attributes = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
    let mut combatant_a = Character::new("", &attributes, 8);
    let mut combatant_b = Character::new("", &attributes, 8);

    // Act
    let duration = {
        let mut combat = Combat::new(&combatant_a, &combatant_b);

        // Fight until either party is unable to combat
        while combat.can_combat(&combatant_a, &combatant_b) {
            combat.apply_round(&mut combatant_a, &mut combatant_b);
        }

        combat.duration
    };

    // Assert
    assert_eq!(combatant_a.life(), 0);
    assert_eq!(combatant_b.life(), 0);
    assert_eq!(duration, 3);
}

#[test]
fn winner_is_declared() {
    // Arrange
    let attributes_a = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
    let attributes_b = hashmap![Attribute::MaxLife => 5, Attribute::Damage => 1];
    let mut combatant_a = Character::new("A", &attributes_a, 8);
    let mut combatant_b = Character::new("B", &attributes_b, 8);

    // Act
    let winner_str;
    {
        let mut combat = Combat::new(&combatant_a, &combatant_b);

        // Fight until either party is unable to combat
        while combat.can_combat(&combatant_a, &combatant_b) {
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
    let monster = MonsterBuilder::new("name", 1, 3).difficulty(1).spawn();
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
