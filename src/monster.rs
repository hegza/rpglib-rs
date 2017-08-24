use super::*;
use serde::*;
use serde_yaml::*;
#[macro_use()]
use serde_derive::*;
use std::collections::HashMap;
use std::iter::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Monster {
    life: i32,
    damage: i32,
    name: String,
    /// Designer defined difficulty
    difficulty: Option<i32>,
}

/// Template monsters represent themed variants of monsters. Template monsters
/// are converted to normal Monsters before instantiation in game world.
#[derive(Serialize, Deserialize)]
pub struct TemplateMonster {
    template: Monster,
    variants: HashMap<Keyword, Variant>,
}

pub type Variant = Vec<Variable>;

impl TemplateMonster {
    pub fn new(template: Monster, variants: HashMap<Keyword, Variant>) -> TemplateMonster {
        TemplateMonster {
            template: template,
            variants: variants,
        }
    }
}

/// Changing properties for a monster of a certain theme
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Variable {
}

pub struct MonsterBuilder {
    monster: Monster,
}

impl MonsterBuilder {
    // Only required elements can go here
    pub fn new(name: &str, damage: i32, life: i32) -> Self {
        MonsterBuilder {
            monster: Monster {
                name: name.to_string(),
                damage: damage,
                life: life,
                difficulty: None,
            },
        }
    }
    pub fn difficulty(mut self, d: i32) -> Self {
        self.monster.difficulty = Some(d);
        self
    }
    // TODO: from template
    // Spawn a copy of the generated monster
    pub fn spawn(&self) -> Monster {
        self.monster.clone()
    }
}

impl Combatant for Monster {
    fn damage(&self) -> i32 {
        self.damage
    }
    fn reduce_life(&mut self, amount: i32) -> i32 {
        self.life -= amount;
        if self.life < 0 {
            self.life = 0;
        }
        self.life
    }
    fn life(&self) -> i32 {
        self.life
    }
    fn can_combat(&self) -> bool {
        self.life > 0
    }
    fn weapon(&self) -> Option<&Equipment> {
        None
    }
}

impl Display for Monster {
    fn name(&self) -> String {
        self.name().clone()
    }
}
