use super::Display;
use super::Equipment;
use inflector::Inflector;

pub trait Combatant: Display {
    fn damage(&self) -> i32;
    fn reduce_life(&mut self, amount: i32) -> i32;
    fn life(&self) -> i32;
    fn can_combat(&self) -> bool;
    fn weapon(&self) -> Option<&Equipment>;
}

/// Combat state, ie. information retained between combat rounds.
pub struct Combat {
    pub duration: i32,
    pub results: Results,
}

pub enum Results {
    Begin { log: String },
    Round { log: String },
    End {
        log: String,
        winner: CombatantId,
        duration: i32,
    },
}

impl Combat {
    pub fn new<T: Combatant>(combatant_a: &T, combatant_b: &T) -> Combat {
        Combat {
            duration: 0,
            results: Results::begin("Combat begins."),
        }
    }
    /// Runs all remaining combat rounds and returns the end result
    pub fn quick_combat<T: Combatant>(&mut self,
                                      combatant_a: &mut T,
                                      combatant_b: &mut T)
                                      -> &Results {
        // Combat has already ended, return latest results
        if let Results::End { log: _, winner: _, duration: _ } = self.results {
            return &self.results;
        }

        // Fight until either party is unable to combat
        while self.can_combat(combatant_a, combatant_b) {
            // Apply rounds and discard results
            self.apply_round(combatant_a, combatant_b);
        }

        // Return end results only
        &self.results
    }
    pub fn apply_round<'a, T: Combatant>(&'a mut self, a: &mut T, b: &mut T) -> &'a Results {
        // Combat has already ended, return latest results
        if let Results::End { .. } = self.results {
            return &self.results;
        }

        // Do combat calculations
        let results = {
            // Gather damage values
            let damage_by_a = a.damage();
            let damage_by_b = b.damage();

            // Deal damage to both combatants based on the others damage
            a.reduce_life(damage_by_b);
            b.reduce_life(damage_by_a);

            match (a.can_combat(), b.can_combat()) {
                // Both can still fight: return round results
                (true, true) => Results::round(a, b, damage_by_a, damage_by_b),
                (true, false) => {
                    Results::end(a,
                                 b,
                                 damage_by_a,
                                 damage_by_b,
                                 CombatantId::A,
                                 self.duration)
                }
                (false, true) => {
                    Results::end(a,
                                 b,
                                 damage_by_a,
                                 damage_by_b,
                                 CombatantId::B,
                                 self.duration)
                }
                // TODO: improve handling of ties
                (false, false) => {
                    Results::end(a,
                                 b,
                                 damage_by_a,
                                 damage_by_b,
                                 CombatantId::B,
                                 self.duration)
                }
            }
        };

        self.duration += 1;

        self.results = results;
        &self.results
    }
    pub fn can_combat(&self, a: &Combatant, b: &Combatant) -> bool {
        let a_can = a.can_combat();
        let b_can = b.can_combat();
        a_can && b_can
    }
}

// TODO: this shouldn't be a part of the public interface
#[derive(Clone, Copy)]
pub enum CombatantId {
    A,
    B,
}

impl Results {
    fn begin(log: &str) -> Results {
        Results::Begin { log: log.to_string() }
    }
    fn round<T: Combatant>(a: &T, b: &T, damage_by_a: i32, damage_by_b: i32) -> Results {
        // Write about the hits
        let a_to_b = Results::hit(a, b, damage_by_a);
        let b_to_a = Results::hit(b, a, damage_by_b);

        let log = a_to_b + &" " + &b_to_a;
        Results::Round { log: log }
    }
    fn end<T: Combatant>(a: &T,
                         b: &T,
                         damage_by_a: i32,
                         damage_by_b: i32,
                         winner_id: CombatantId,
                         duration: i32)
                         -> Results {
        // Write about the hits
        let a_to_b = Results::hit(a, b, damage_by_a);
        let b_to_a = Results::hit(b, a, damage_by_b);

        // Write about the winner
        let winner = winner_id.to_combatant(a, b);
        let winner_str = format!("{} wins the combat!", &winner.name()).to_sentence_case();

        let log = a_to_b + &" " + &b_to_a + &" " + &winner_str;
        Results::End {
            log: log,
            winner: winner_id,
            duration: duration,
        }
    }
    fn hit<T: Combatant>(a: &T, b: &T, damage: i32) -> String {
        let a_weapon_name = match a.weapon() {
            Some(item) => item.name(),
            None => "an appendage".to_owned(),
        };

        let mut a_to_b = format!("{0} hits {1} with {2} for",
                                 a.name(),
                                 b.name(),
                                 a_weapon_name);
        a_to_b = a_to_b.to_sentence_case();
        a_to_b += &" ";
        a_to_b += &format!("{0} damage ({1} remains).", damage, b.life());
        if !a.can_combat() {
            a_to_b += &" ";
            a_to_b += &format!("{0} dies!", a.name()).to_sentence_case();
        }
        a_to_b
    }
}

impl CombatantId {
    pub fn to_combatant<'a, T: Combatant>(&self, a: &'a T, b: &'a T) -> &'a Combatant {
        match self {
            &CombatantId::A => a,
            &CombatantId::B => b,
        }
    }
}
