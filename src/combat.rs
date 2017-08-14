use inflector::Inflector;
use super::item::EquipItem;

pub trait Combatant {
    fn damage(&self) -> i32;
    fn reduce_life(&mut self, amount: i32) -> i32;
    fn life(&self) -> i32;
    fn can_combat(&self) -> bool;
    fn english_name(&self) -> String;
    fn weapon(&self) -> Option<&EquipItem>;
}

pub struct Combat<'a> {
    combatant_a: &'a mut Combatant,
    combatant_b: &'a mut Combatant,
    combat_duration: i32,
    // TODO: move to EndResults
    winner: Option<CombatantId>,
}

enum CombatantId {
    A,
    B,
}

pub struct RoundResults {
    pub english_log: Vec<String>,
}

pub struct EndResults {
    pub combat_duration: i32,
    pub english_log: Vec<String>,
}

impl<'a> Combat<'a> {
    pub fn new(combatant_a: &'a mut Combatant, combatant_b: &'a mut Combatant) -> Combat<'a> {
        Combat {
            combatant_a: combatant_a,
            combatant_b: combatant_b,
            combat_duration: 0,
            winner: None,
        }
    }
    /// Runs all remaining combat rounds and returns the combat result
    pub fn quick_combat(&mut self) -> EndResults {
        // Fight until either party is unable to combat
        while self.can_combat() {
            self.apply_round();
        }
        self.end_results()
    }
    pub fn apply_round(&mut self) -> RoundResults {
        // Do combat calculations
        let damage_by_a;
        let damage_by_b;
        {
            // Gather damage values
            damage_by_a = self.combatant_a.damage();
            damage_by_b = self.combatant_b.damage();

            // Deal damage to both combatants based on the others damage
            self.combatant_a.reduce_life(damage_by_b);
            self.combatant_b.reduce_life(damage_by_a);

            self.combat_duration += 1;

            // Check combat status (ie. winner)
            let (a, b) = (self.combatant_a.can_combat(), self.combatant_b.can_combat());
            if a != b {
                if a {
                    self.winner = Some(CombatantId::A);
                } else {
                    self.winner = Some(CombatantId::B);
                }
            }
        }

        RoundResults { english_log: self.log_round(damage_by_a, damage_by_b) }
    }
    fn log_round(&self, damage_by_a: i32, damage_by_b: i32) -> Vec<String> {
        let a = &self.combatant_a;
        let b = &self.combatant_b;
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

        let mut lines = vec![a_to_b + &" " + &b_to_a];
        if let Some(winner) = self.winner() {
            lines.push(format!("{0} wins the combat!", winner.english_name()).to_sentence_case());
        }
        lines
    }
    pub fn winner(&self) -> Option<&Combatant> {
        match self.winner {
            Some(CombatantId::A) => Some(self.combatant_a),
            Some(CombatantId::B) => Some(self.combatant_b),
            None => None,
        }
    }
    pub fn end_results(&mut self) -> EndResults {
        EndResults {
            combat_duration: self.combat_duration,
            english_log: vec![],
        }
    }
    pub fn can_combat(&self) -> bool {
        self.combatant_a.can_combat() && self.combatant_b.can_combat()
    }
}
