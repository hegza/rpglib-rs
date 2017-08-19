mod item;

use super::*;
use super::combat::*;
use super::item::*;

#[test]
fn combat_works() {
    /// Arrange
    let attributes = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
    let mut combatant_a = Character::new("", &attributes, 8);
    let mut combatant_b = Character::new("", &attributes, 8);

    /// Act
    let combat_duration = {
        let mut combat = Combat::new();

        // Fight until either party is unable to combat
        while combat.can_combat(&combatant_a, &combatant_b) {
            combat.apply_round(&mut combatant_a, &mut combatant_b);
        }

        combat.combat_duration
    };

    /// Assert
    assert_eq!(combatant_a.life(), 0);
    assert_eq!(combatant_b.life(), 0);
    assert_eq!(combat_duration, 3);
}

#[test]
fn winner_is_declared() {
    /// Arrange
    let attributes_a = hashmap![Attribute::MaxLife => 3, Attribute::Damage => 1];
    let attributes_b = hashmap![Attribute::MaxLife => 5, Attribute::Damage => 1];
    let mut combatant_a = Character::new("A", &attributes_a, 8);
    let mut combatant_b = Character::new("B", &attributes_b, 8);

    /// Act
    let winner = {
        let mut combat = Combat::new();

        // Fight until either party is unable to combat
        while combat.can_combat(&combatant_a, &combatant_b) {
            // Assert: there must not be a winner while the combatants can combat
            assert!(combat.winner(&combatant_a, &combatant_b).is_none());

            combat.apply_round(&mut combatant_a, &mut combatant_b);
        }

        let results = combat.end_results().unwrap();
        let winner = results.winner(&combatant_a, &combatant_b).unwrap();
        winner.english_name()
    };

    /// Assert
    assert_eq!(winner, combatant_b.english_name());
}
