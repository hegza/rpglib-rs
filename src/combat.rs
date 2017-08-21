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
    pub combat_duration: i32,
    pub last_results: CombatResults,
}

#[derive(Clone, Copy)]
enum CombatantId {
    A,
    B,
}

/// Results for this combat round.
pub struct CombatResults {
    pub english_log: Vec<String>,
    winner: Option<CombatantId>,
}

impl Combat {
    pub fn new() -> Combat {
        Combat {
            combat_duration: 0,
            last_results: CombatResults {
                english_log: vec!["Combat begins.".to_owned()],
                winner: None,
            },
        }
    }
    /// Runs all remaining combat rounds and returns the combat result
    pub fn quick_combat(&mut self,
                        combatant_a: &mut Combatant,
                        combatant_b: &mut Combatant)
                        -> CombatResults {
        // Fight until either party is unable to combat
        while self.can_combat(combatant_a, combatant_b) {
            // Apply rounds and discard results
            self.apply_round(combatant_a, combatant_b);
        }

        // Return end results only
        self.end_results().unwrap()
    }
    pub fn apply_round<'a>(&'a mut self,
                           combatant_a: &mut Combatant,
                           combatant_b: &mut Combatant)
                           -> &'a CombatResults {
        // Do combat calculations
        let a = combatant_a;
        let b = combatant_b;
        let damage_by_a;
        let damage_by_b;
        let mut winner = None;
        {
            // Gather damage values
            damage_by_a = a.damage();
            damage_by_b = b.damage();

            // Deal damage to both combatants based on the others damage
            a.reduce_life(damage_by_b);
            b.reduce_life(damage_by_a);

            // Check combat status (ie. winner)
            let (a_can_combat, b_can_combat) = (a.can_combat(), b.can_combat());
            if a_can_combat != b_can_combat {
                if a_can_combat {
                    winner = Some(CombatantId::A);
                } else {
                    winner = Some(CombatantId::B);
                }
            }
        }

        self.combat_duration += 1;

        let results = CombatResults::from_combat(a, b, damage_by_a, damage_by_b, winner);
        self.last_results = results;
        &self.last_results
    }
    pub fn can_combat(&self, a: &Combatant, b: &Combatant) -> bool {
        let a_can = a.can_combat();
        let b_can = b.can_combat();
        a_can && b_can
    }
    pub fn end_results(&self) -> Option<CombatResults> {
        if let Some(winner) = self.last_results.winner {
            return Some(CombatResults::from_winner(winner));
        }
        None
    }
    pub fn winner<'a>(&'a self, a: &'a Combatant, b: &'a Combatant) -> Option<&'a Combatant> {
        match self.last_results.winner {
            Some(CombatantId::A) => return Some(a),
            Some(CombatantId::B) => return Some(b),
            None => return None,
        }
    }
}

impl CombatResults {
    fn from_winner(winner: CombatantId) -> CombatResults {
        CombatResults {
            english_log: vec![],
            winner: Some(winner),
        }
    }
    fn from_combat(a: &Combatant,
                   b: &Combatant,
                   damage_by_a: i32,
                   damage_by_b: i32,
                   winner: Option<CombatantId>)
                   -> CombatResults {
        let mut results = CombatResults {
            english_log: vec![],
            winner: None,
        };

        let a_weapon_name = match a.weapon() {
            Some(item) => item.english_name(),
            None => "an appendage".to_owned(),
        };
        let b_weapon_name = match b.weapon() {
            Some(item) => item.english_name(),
            None => "an appendage".to_owned(),
        };
        let mut a_to_b = format!("{0} hits {1} with {2} for",
                                 a.english_name(),
                                 b.english_name(),
                                 a_weapon_name);
        a_to_b = a_to_b.to_sentence_case();
        a_to_b += &" ";
        a_to_b += &format!("{0} damage ({1} remains).", damage_by_a, b.life());
        if !a.can_combat() {
            a_to_b += &" ";
            a_to_b += &format!("{0} dies!", a.english_name()).to_sentence_case();
        }

        let mut b_to_a = format!("{0} hits {1} with {2} for",
                                 b.english_name(),
                                 a.english_name(),
                                 b_weapon_name);
        b_to_a = b_to_a.to_sentence_case();
        b_to_a += &" ";
        b_to_a += &format!("{0} damage ({1} remains).", damage_by_b, a.life());
        if !b.can_combat() {
            b_to_a += &" ";
            b_to_a += &format!("{0} dies!", b.english_name()).to_sentence_case();
        }

        results.winner = winner.clone();
        let mut lines = vec![a_to_b + &" " + &b_to_a];
        if let Some(winner) = winner {
            let winner = CombatResults::id_to_winner(a, b, winner);
            lines.push(format!("{0} wins the combat!", winner.english_name()).to_sentence_case());
        }
        results.english_log.extend(lines);
        results
    }
    fn id_to_winner<'a>(a: &'a Combatant, b: &'a Combatant, winner: CombatantId) -> &'a Combatant {
        match winner {
            CombatantId::A => a,
            CombatantId::B => b,
        }
    }
    pub fn winner<'a>(&'a self, a: &'a Combatant, b: &'a Combatant) -> Option<&'a Combatant> {
        match self.winner {
            None => None,
            Some(CombatantId::A) => Some(a),
            Some(CombatantId::B) => Some(b),
        }
    }
}
