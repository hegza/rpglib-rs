use super::*;
use std::collections::HashMap;
use std::cmp::max;
use dungeon::generator::Evaluate;
use std::iter::FromIterator;
use item::Equipment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monster {
    life: i32,
    damage: i32,
    name: String,
    /// Designer defined difficulty
    difficulty: Option<usize>,
    keywords: Vec<Keyword>,
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
                keywords: vec![],
            },
        }
    }
    pub fn difficulty(mut self, d: usize) -> Self {
        self.monster.difficulty = Some(d);
        self
    }
    pub fn keywords(mut self, keywords: &[&str]) -> Self {
        let keywords: Vec<Keyword> = keywords
            .iter()
            .map(|x| Keyword { id: x.to_string() })
            .collect();
        self.monster.keywords.extend(keywords);
        self
    }
    pub fn keyword(mut self, keyword: &str) -> Self {
        self.monster.keywords.push(Keyword {
            id: keyword.to_string(),
        });
        self
    }
    // TODO: from template
    // Spawn a copy of the generated monster
    pub fn spawn(&self) -> Monster {
        self.monster.clone()
    }
}

lazy_static! {
    static ref DEFAULT_MONSTER_WEAPON: Equipment = equipment("fist", 1, Slot::Hand, vec![]).build();
}

impl Combatant for Monster {
    fn best_weapon(&self) -> &Equipment {
        &DEFAULT_MONSTER_WEAPON
    }
    fn damage(&self) -> i32 {
        self.damage
    }
    fn action_buffer(&self) -> ActionBuffer {
        ActionBuffer::default()
    }
    fn set_life(&mut self, amount: i32) -> i32 {
        self.life = max(amount, 0);
        self.life
    }
    fn life(&self) -> i32 {
        self.life
    }
    fn can_combat(&self) -> bool {
        self.life > 0
    }
}

impl Display for Monster {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl<'a> Evaluate for Monster {
    fn theme(&self) -> &[Keyword] {
        self.keywords.as_slice()
    }
    fn difficulty(&self) -> usize {
        self.difficulty.expect(&format!(
            "monsters without difficulty may not be included in a dungeon: {}",
            &self.name
        ))
    }
}

impl AsRef<Monster> for Monster {
    fn as_ref(&self) -> &Self {
        self
    }
}
