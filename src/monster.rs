use super::*;
use serde::*;
use serde_yaml::*;
#[macro_use()]
use serde_derive::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Monster {
    current_life: i32,
    damage: i32,
    english_name: String,
}

impl Monster {
    pub fn new(english_name: &str, damage: i32, life: i32) -> Monster {
        Monster {
            current_life: life,
            damage: damage,
            english_name: english_name.to_owned(),
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

pub struct MonsterBuilder {
    monster: Monster,
}

impl MonsterBuilder {
    // Only required elements can go here
    pub fn new(name: &str, damage: i32, life: i32) -> Self {
        MonsterBuilder { monster: Monster::new(name, damage, life) }
    }
    pub fn spawn(&self) -> Monster {
        self.monster.clone()
    }
}

/// Template monsters represent themed variants of monsters. Template monsters
/// are converted to normal Monsters before instantiation in game world.
#[derive(Serialize, Deserialize)]
pub struct TemplateMonster {
    allowed_themes: Vec<Keyword>,
    inner: Monster,
}
