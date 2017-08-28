mod results;

pub use self::results::Results;

use self::results::*;
use super::Display;
use std::cmp::max;

pub trait Combatant: Display {
    fn life(&self) -> i32;
    fn set_life(&mut self, amount: i32) -> i32;
    fn can_combat(&self) -> bool;
    fn action_buffer(&self) -> ActionBuffer;
    // TODO: do something about stamina
    fn damage(&self) -> i32;
}

#[derive(Clone)]
pub struct ActionBuffer {
    // (action, duration)
    actions: Vec<(Action, usize)>,
    max_actions: usize,
}

impl ActionBuffer {
    pub fn new(max_actions: usize) -> ActionBuffer {
        ActionBuffer {
            actions: vec![],
            max_actions,
        }
    }
    pub fn duration_reserved(&self) -> usize {
        self.actions.iter().map(|&(_, d)| d).sum()
    }
    pub fn duration_free(&self) -> usize {
        self.max_actions - self.duration_reserved()
    }
    pub fn push(&mut self, act: &Action, duration: usize) -> bool {
        // Early out if cannot add
        if self.duration_free() < duration {
            return false;
        }

        // Add the action to buffer
        self.actions.push((act.clone(), duration));
        true
    }
    pub fn clear(&mut self) {
        self.actions.clear();
    }
    pub fn count(&self, action: &Action) -> usize {
        self.actions
            .iter()
            .filter(|&&(ref act, _)| act == action)
            .count()
    }
    pub fn duration_of(&self, action: &Action) -> usize {
        self.actions
            .iter()
            .filter(|&&(ref act, _)| act == action)
            .map(|&(_, ref d)| d)
            .sum()
    }
}

impl Default for ActionBuffer {
    fn default() -> ActionBuffer {
        ActionBuffer {
            actions: vec![(Action::Attack, 1)],
            max_actions: 1,
        }
    }
}

/// Things that the combatants may do.
#[derive(Clone, Eq, PartialEq)]
pub enum Action {
    Evade,
    //Block,
    Attack,
}

/// All that actually happened (to a target).
#[derive(Clone, Eq, PartialEq)]
enum Outcome {
    Miss,
    //Block,
    Hit(i32),
    //Crit(i32),
    Kill,
}

/// Combat state, ie. information retained between combat rounds.
pub struct Combat {
    pub duration: i32,
    pub results: Results,
}

impl Combat {
    pub fn new<T: Combatant, U: Combatant>(combatant_a: &T, combatant_b: &U) -> Combat {
        Combat {
            duration: 0,
            results: ResultsBuilder::new(combatant_a, combatant_b).build_begin(),
        }
    }
    /// Runs all remaining combat rounds and returns the end result
    pub fn quick_combat<T: Combatant, U: Combatant>(
        &mut self,
        combatant_a: &mut T,
        combatant_b: &mut T,
    ) -> &Results {
        // Combat has already ended, return latest results
        if let Results::End { .. } = self.results {
            return &self.results;
        }

        // Fight until either party is unable to combat
        while Combat::can_combat(combatant_a, combatant_b) {
            // Apply rounds and discard results
            self.apply_round(combatant_a, combatant_b);
        }

        // Return last results only (ie. end results)
        &self.results
    }
    /// Resolves one combat round and records results to self.
    pub fn apply_round<'a, T: Combatant, U: Combatant>(
        &'a mut self,
        a: &mut T,
        b: &mut U,
    ) -> &'a Results {
        // Combat has already ended, return latest results
        if let Results::End { .. } = self.results {
            return &self.results;
        }

        // Do combat calculations
        let results = {
            use Action::*;

            let a_buffer = &a.action_buffer();
            let b_buffer = &a.action_buffer();

            // Count number of different actions
            let num_atks_by_a = a_buffer.count(&Attack) as i32;
            let num_atks_by_b = b_buffer.count(&Attack) as i32;
            let num_evas_by_a = a_buffer.count(&Evade) as i32;
            let num_evas_by_b = b_buffer.count(&Evade) as i32;

            // Resolve outcomes
            let num_hits_to_a = max(num_atks_by_b - num_evas_by_a, 0);
            let num_hits_to_b = max(num_atks_by_a - num_evas_by_b, 0);
            let num_misses_to_a = num_atks_by_b - num_hits_to_a;
            let num_misses_to_b = num_atks_by_a - num_hits_to_b;

            use self::Outcome::*;
            let mut outcomes_a = vec![Miss; num_misses_to_a as usize];
            for _ in 0..num_hits_to_a {
                outcomes_a.push(Hit(b.damage()));
            }
            let mut outcomes_b = vec![Miss; num_misses_to_b as usize];
            for _ in 0..num_hits_to_b {
                outcomes_b.push(Hit(a.damage()));
            }
            if !a.can_combat() {
                outcomes_a.push(Kill);
            }
            if !b.can_combat() {
                outcomes_b.push(Kill);
            }

            // TODO: make combat cooler by taking into account hits with each item used as a weapon.
            // TODO: use outcomes to do the calculation
            // Resolve a -> b
            let a_life = a.life();
            a.set_life(a_life - num_hits_to_a * b.damage());
            // Resolve b -> a
            let b_life = b.life();
            b.set_life(b_life - num_hits_to_b * a.damage());

            let builder = ResultsBuilder::new(a, b).write_round(&outcomes_a, &outcomes_b);
            match (a.can_combat(), b.can_combat()) {
                (true, true) => builder.build_round(),
                (true, false) => builder.build_end(CombatantId::A, self.duration),
                (false, true) => builder.build_end(CombatantId::B, self.duration),
                // TODO: improve handling of ties
                (false, false) => builder.build_end(CombatantId::B, self.duration),
            }
        };

        self.duration += 1;

        self.results = results;
        &self.results
    }
    pub fn can_combat<T: Combatant, U: Combatant>(a: &T, b: &U) -> bool {
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


impl CombatantId {
    pub fn to_combatant<'a, T: Combatant, U: Combatant>(
        &self,
        a: &'a T,
        b: &'a U,
    ) -> &'a Combatant {
        match self {
            &CombatantId::A => a,
            &CombatantId::B => b,
        }
    }
}
