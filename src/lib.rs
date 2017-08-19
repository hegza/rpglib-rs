extern crate inflector;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate try_from;

// Keep the #[macro use] util first
#[macro_use]
mod utils;
mod item;
mod combat;
mod reward;
mod character;
mod theme;

#[cfg(test)]
mod tests;

pub use item::*;
pub use combat::*;
pub use reward::*;
pub use character::*;
pub use utils::*;
pub use theme::*;

pub struct Monster {
    current_life: i32,
    damage: i32,
    english_name: String,
    reward: Option<Reward>,
}

impl Monster {
    pub fn new(english_name: &str, damage: i32, life: i32, reward: Option<Reward>) -> Monster {
        Monster {
            current_life: life,
            damage: damage,
            english_name: english_name.to_owned(),
            reward: reward,
        }
    }
}

impl Combatant for Monster {
    fn damage(&self) -> i32 {
        self.damage
    }
    fn reduce_life(&mut self, amount: i32) -> i32 {
        self.current_life -= amount;
        if self.current_life < 0 {
            self.current_life = 0;
        }
        self.current_life
    }
    fn life(&self) -> i32 {
        self.current_life
    }
    fn can_combat(&self) -> bool {
        self.current_life > 0
    }
    fn weapon(&self) -> Option<&Equipment> {
        None
    }
}

impl Display for Monster {
    fn english_name(&self) -> String {
        self.english_name.clone()
    }
}

impl YieldReward for Monster {
    fn reward(&self) -> Option<Reward> {
        match self.reward {
            None => None,
            Some(ref reward) => {
                match reward {
                    &Reward::Item(ref item) => {
                        let clone: Item = (*item).clone();
                        return Some(Reward::Item(clone));
                    }
                }
            }
        }
    }
}

pub trait Display {
    fn english_name(&self) -> String;
}
