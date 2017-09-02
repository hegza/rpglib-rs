use super::*;
use rand::{thread_rng, Rng};
use display::{Display, DisplayWeapon};
use inflector::Inflector;
use rustache::*;
use std::io::Cursor;

pub enum Results {
    Begin { log: String },
    Round { log: String },
    End {
        log: String,
        winner: CombatantId,
        duration: i32,
    },
}

/// The results builder liberally refers to a and b. A is the player, B is the
/// opponent.
pub struct ResultsBuilder<'a, T, U> where T: Combatant, U: Combatant, T: 'a, U: 'a {
    template_log: String,
    data: HashBuilder<'a>,
    a: &'a T,
    b: &'a U,
}

impl<'a, T, U> ResultsBuilder<'a, T, U> where T: Combatant, U: Combatant {
    // Constructor, include only that what is always needed or always available.
    pub fn new(a: &'a T, b: &'a U) -> ResultsBuilder<'a, T, U> {
        let a_weapon = a.best_weapon();
        let b_weapon = b.best_weapon();
        let str_builder = HashBuilder::new()
                .insert("a_name", a.name())
                .insert("b_name", b.name())
                .insert("a_weapon", a_weapon.name())
                .insert("a_weapon_action", a_weapon.display_offensive_action_1st())
                .insert("b_weapon", b_weapon.name())
                .insert("b_weapon_action", b_weapon.display_offensive_action_2nd());
        ResultsBuilder {
            template_log: String::new(),
            data: str_builder,
            a, b
        }
    }
    // Builder functions (finalizers)
    pub fn build_begin(mut self) -> Results {
        // The begin always looks the same, set the log to that.
        self.template_log = BEGIN.to_owned();
        Results::Begin { log: self.fill_template() }
    }
    pub fn build_round(self) -> Results {
        Results::Round { log: self.fill_template() }
    }
    pub fn build_end(self, winner: CombatantId, duration: i32) -> Results {
        Results::End {
            log: self.fill_template(),
            winner: winner,
            duration: duration,
        }
    }
    /// Fills in the variables into the template.
    fn fill_template(&self) -> String {
        let mut out = Cursor::new(Vec::new());
        self.data.render(&self.template_log, &mut out).unwrap();
        String::from_utf8(out.into_inner()).unwrap()
    }
    pub fn write_round(mut self, a_outcomes: &Vec<Outcome>, b_outcomes: &Vec<Outcome>) -> ResultsBuilder<'a, T, U> {
        let (mut sentences, a_killed, b_killed) = ResultsBuilder::<T, U>::outcome_sentences(a_outcomes, b_outcomes);

        // Randomize the order of sentences
        {
            let slice: &mut [String] = sentences.as_mut_slice();
            thread_rng().shuffle(slice);
        }

        // Add the kill sentence to the end
        if let Some(s) = ResultsBuilder::<T, U>::kill_sentence(a_killed, b_killed) {
            sentences.push(s);
        }
        self.template_log.push_str(&sentences.join(" "));
        self
    }
    // Internals
    fn outcome_sentences(a_outcomes: &Vec<Outcome>, b_outcomes: &Vec<Outcome>) -> (Vec<String>, bool, bool) {
        let mut sentences = Vec::with_capacity(a_outcomes.len() + b_outcomes.len());
        let mut a_killed = false;
        for outcome in a_outcomes {
            match *outcome {
                Outcome::Miss => {
                    sentences.push(THEY_MISS.to_owned());
                },
                Outcome::Hit(_) => {
                    sentences.push(THEY_HIT.to_owned());
                },
                Outcome::Killed => {
                    a_killed = true;
                    break;
                }
            }
        }
        let mut b_killed = false;
        for outcome in b_outcomes {
            match *outcome {
                Outcome::Miss => {
                    sentences.push(YOU_MISS.to_owned());
                },
                Outcome::Hit(_) => {
                    sentences.push(YOU_HIT.to_owned());
                },
                Outcome::Killed => {
                    b_killed = true;
                    break;
                }
            }
        }
        (sentences, a_killed, b_killed)
    }
    fn kill_sentence(a_killed: bool, b_killed: bool) -> Option<String> {
        match (a_killed, b_killed) {
            (true, true) =>
                Some(BOTH_KILL.to_owned()),
            (true, false) =>
                Some(THEY_KILL.to_owned()),
            (false, true) =>
                Some(YOU_KILL.to_owned()),
            _ => None,
        }
    }
}



static BEGIN: &str = "The {{b_name}} notices you and attacks.";
static YOU_MISS: &str = "You attempt to {{a_weapon_action}} the {{b_name}} with the {{a_weapon}} but \
                        miss.";
static THEY_MISS: &str = "The {{b_name}} attempts to {{b_weapon_action}} you with a {{b_weapon}} but \
                         misses.";
static YOU_HIT: &str = "You {{a_weapon_action}} the {{b_name}} with the {{a_weapon}}, wounding it.";
static THEY_HIT: &str = "The {{b_name}} {{b_weapon_action}} you with a {{b_weapon}}, wounding you.";
static YOU_KILL: &str = "You {{a_weapon_action}} the {{b_name}} with the {{a_weapon}} until you are \
                        certain that you are the only living thing in the room. You are safe now.";
static THEY_KILL: &str = "The {{b_name}} {{b_weapon_action}} you with their {{b_weapon}}, causing you \
                         to feel lightheaded. You suddenly lose consciousness. You die.";
static BOTH_KILL: &str = "The {{b_name}} {{b_weapon_action}} you with their {{b_weapon}}, causing you \
                         to feel lightheaded. You {{a_weapon_action}} the {{b_name}} with the \
                         {{a_weapon}}, causing yet another untimely death. Soon after the \
                         {{b_name}}'s death you suddenly collapse. Despite your best efforts, you \
                         are unable to stop the hemorrhaging and quickly (try to) make peace with \
                         your god.";

/*
## Example (25.8.-17)
BEGIN
The goblin notices you and attacks.

YOU_MISS / THEY_MISS
You attempt to bash the goblin with the stick but miss. The goblin attempts
to hit you with a fist but misses.

YOU_HIT / THEY_HIT
You bash the goblin with the stick, wounding them. The goblin hits you with a
fist, wounding you.

YOU_KILL
<goblin hits>. You bash the goblin with the stick until you are certain that 
you are the only living thing in the room. You are safe now.

THEY_KILL
<player hits>. The goblin hits you with their fist, causing you to feel 
lightheaded. You suddenly lose consciousness. You die.

BOTH_KILL
The goblin hits you with their fist, causing you to feel lightheaded. You bash
the goblin with the stick, causing yet another untimely death. Soon after the 
goblin's death you suddenly collapse. Despite your best attempts, you are 
unable to stop the hemorrhaging and quickly (try to) make peace with your god.
*/

/*
## Notes
the -> definite article
randomize the order of sentences per round, unless a party has died
### Flow
Begin -> N x Round -> End
Begin: 1 sentence
Round: N + M sentences, O variants each (N,M <= number of actions per entity)
End:
*/
