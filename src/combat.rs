
pub trait Combatant {
    fn damage(&self) -> i32;
    fn reduce_life(&mut self, amount: i32) -> i32;
    fn life(&self) -> i32;
    fn can_combat(&self) -> bool;
}

pub struct Combat<'a> {
    combatant_a: &'a mut Combatant,
    combatant_b: &'a mut Combatant,
    combat_duration: i32,
}

pub struct RoundResults {
    pub english_log: String,
}

pub struct EndResults {
    pub combat_duration: i32,
    pub english_log: String,
}

impl<'a> Combat<'a> {
    pub fn new(combatant_a: &'a mut Combatant, combatant_b: &'a mut Combatant) -> Combat<'a> {
        Combat {
            combatant_a: combatant_a,
            combatant_b: combatant_b,
            combat_duration: 0,
        }
    }
    pub fn apply_round(&mut self) -> RoundResults {
        // Gather damage values
        let damage_a = self.combatant_a.damage();
        let damage_b = self.combatant_b.damage();

        // Deal damage to both combatants based on the others damage
        self.combatant_a.reduce_life(damage_b);
        self.combatant_b.reduce_life(damage_a);

        self.combat_duration += 1;

        RoundResults { english_log: "".to_owned() }
    }
    pub fn end_combat(&mut self) -> EndResults {
        EndResults {
            combat_duration: self.combat_duration,
            english_log: "".to_owned(),
        }
    }
    pub fn can_combat(&self) -> bool {
        self.combatant_a.can_combat() && self.combatant_b.can_combat()
    }
}
