use rand::{Rng, SeedableRng, StdRng};
use super::theme::Keyword;
use range::Range;

pub struct Dungeon(Vec<Room>);
pub struct Room;

impl Dungeon {
    pub fn generate(seed: Seed, generator: &Generator) {
        // TODO: split the map into archs
        // TODO: split the archs into areas
        // TODO: split the areas into rooms (mains / optionals)
    }
}

struct Generator {
    monster_pool: Vec<Monster>,
    template_monster_pool: Vec<Monster>,
    theme_keyword_pool: Vec<Keyword>,
    // Archs define the style of the dungeon
    arch_count: usize,
    // Areas define thematic closures
    area_count: Range<usize>,
    // Rooms are for encounters with monsters, treasures and altars
    main_path_room_count: usize,
}
