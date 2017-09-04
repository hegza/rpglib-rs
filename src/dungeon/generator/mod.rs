#[cfg(test)]
mod tests;
pub mod evaluate;

use theme::Keyword;
use rand::{Rng, SeedableRng, StdRng};
use range::Range;
use monster::*;
use super::*;
use std::f32;
pub use dungeon::generator::evaluate::*;
// temp
use display::Display;

pub struct Generator<'a> {
    monster_pool: &'a [Monster],
    template_monster_pool: &'a [TemplateMonster],
    theme_keyword_pool: &'a [Keyword],
    dungeon_keyword_count: usize,
    arch_keyword_count: usize,
    area_keyword_count: usize,
    /// Archs define the style of the dungeon.
    arch_count: usize,
    /// no. of areas in an arch. Areas define thematic closures.
    num_areas_in_arch: Range,
    /// no. of rooms in an area. Rooms are for encounters with monsters, treasures and altars.
    num_main_rooms_in_area: Range,
    force_first_room_empty: bool,
}

// TOOD: reimplement with lifetime subtyping so that no copies of Keywords have
// to be made
impl<'a> Generator<'a> {
    pub fn new(
        monster_pool: &'a [Monster],
        template_monster_pool: &'a [TemplateMonster],
        theme_keyword_pool: &'a [Keyword],
        dungeon_keyword_count: usize,
        arch_keyword_count: usize,
        area_keyword_count: usize,
        arch_count: usize,
        num_areas_in_arch: Range,
        num_main_rooms_in_area: Range,
        force_first_room_empty: bool,
    ) -> Generator<'a> {
        Generator {
            monster_pool: monster_pool,
            template_monster_pool: template_monster_pool,
            theme_keyword_pool: theme_keyword_pool,
            dungeon_keyword_count: dungeon_keyword_count,
            arch_keyword_count: arch_keyword_count,
            area_keyword_count: area_keyword_count,
            arch_count: arch_count,
            num_areas_in_arch: num_areas_in_arch,
            num_main_rooms_in_area: num_main_rooms_in_area,
            force_first_room_empty,
        }
    }

    pub fn generate(&'a self, seed: &'a [usize]) -> Dungeon {
        // Rename binding for conciseness
        let g = self;

        // Initialize RNG with seed
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        // Select keywords for the whole dungeon dungeon
        let dungeon_keywords = rng.choose_many(g.dungeon_keyword_count, g.theme_keyword_pool);

        // Split the map into arches
        let arches = g.generate_arches(dungeon_keywords.as_slice(), &mut rng);

        // Construct the final dungeon from the intermediaries
        let mut rooms = vec![];
        for arch in &arches {
            for area in &arch.areas {
                let rooms_in_area: Vec<Room> = area.rooms.iter().map(|room| room.clone()).collect();
                rooms.extend(rooms_in_area);
            }
        }
        if self.force_first_room_empty {
            rooms[0].monster = None;
        }

        let mut dungeon = Dungeon::new(rooms);
        // Generate paths
        for i in 0..dungeon.rooms.len() - 1 {
            dungeon.create_passage(i, CompassPoint::East, i + 1);
        }

        dungeon
    }
    fn generate_arches<'b>(
        &'a self,
        keyword_pool: &'b [Keyword],
        mut rng: &'b mut StdRng,
    ) -> Vec<Arch> {
        let g = self;

        let mut arches = Vec::with_capacity(g.arch_count);
        for arch_idx in 0..g.arch_count {
            let difficulty_min = arch_idx as f32 / g.arch_count as f32;
            let difficulty_max = (arch_idx + 1) as f32 / g.arch_count as f32;

            let arch_keywords = rng.choose_many(g.arch_keyword_count, keyword_pool);

            // Create the arch and areas
            let r = g.num_areas_in_arch;
            let num_areas_in_arch = rng.gen_range(r.offset, r.offset + r.length);
            let arch = Arch::new(g.generate_areas(
                num_areas_in_arch,
                difficulty_min,
                difficulty_max,
                arch_keywords.as_slice(),
                &mut rng,
            ));
            arches.push(arch)
        }
        arches
    }
    fn generate_areas<'b>(
        &'a self,
        count: usize,
        difficulty_min: f32,
        difficulty_max: f32,
        keyword_pool: &'b [Keyword],
        mut rng: &'b mut StdRng,
    ) -> Vec<Area> {
        let g = self;

        let mut areas = Vec::with_capacity(count);
        for area_idx in 0..count {
            let area_difficulty_min = difficulty_min +
                (difficulty_max - difficulty_min) * (area_idx as f32 / count as f32);
            let area_difficulty_max = difficulty_min +
                (difficulty_max - difficulty_min) * ((area_idx as f32 + 1.) / count as f32);
            let area_keywords = rng.choose_many(g.area_keyword_count, keyword_pool);

            // Create the area and rooms
            let r = g.num_main_rooms_in_area;
            let num_main_rooms_in_area = rng.gen_range(r.offset, r.offset + r.length);
            let area = Area::new(g.generate_rooms(
                num_main_rooms_in_area,
                area_difficulty_min,
                area_difficulty_max,
                area_keywords.as_slice(),
                &mut rng,
            ));
            areas.push(area);
        }
        areas
    }
    fn generate_rooms<'b>(
        &'a self,
        count: usize,
        area_difficulty_min: f32,
        area_difficulty_max: f32,
        keyword_pool: &'b [Keyword],
        rng: &'b mut StdRng,
    ) -> Vec<Room> {
        let mut rooms = Vec::with_capacity(count);
        for room_idx in 0..count {
            let room_difficulty = area_difficulty_min +
                (area_difficulty_max - area_difficulty_min) *
                    ((room_idx as f32 + 1.) / count as f32);
            let keyword = rng.choose(keyword_pool).unwrap();

            // Create the area and rooms
            rooms.push(self.generate_room(
                vec![keyword].as_slice(),
                room_difficulty,
                rng,
            ));
        }
        rooms
    }
    fn generate_room(&self, keywords: &[&Keyword], difficulty: f32, rng: &mut StdRng) -> Room {
        Room::new(
            keywords[0],
            Some(self.generate_monster(difficulty, keywords, rng)),
        )
    }
    fn generate_monster(
        &self,
        ambient_difficulty: f32,
        ambient_theme: &[&Keyword],
        rng: &mut StdRng,
    ) -> Monster {
        let by_fitness: Vec<(f32, &Monster)> =
            rank_by_fitness(self.monster_pool, ambient_difficulty, ambient_theme);

        eprintln!(
            "d: {:.*}, th: {} -> {:?}",
            2,
            ambient_difficulty,
            ambient_theme.first().unwrap().id,
            by_fitness
                .iter()
                .map(|&(f, ref m)| (f, m.name()))
                .collect::<Vec<(f32, String)>>()
        );

        let total_fitness: f32 = by_fitness.iter().map(|&(f, _)| f).sum();

        // Return what is chosen by fitness
        if total_fitness > 0.01 {
            return rng.choose_weighted(&by_fitness, total_fitness);
        }

        let by_difficulty: Vec<(f32, &Monster)> =
            rank_by_difficulty(self.monster_pool, ambient_difficulty);
        let total_fitness: f32 = by_difficulty.iter().map(|&(f, _)| f).sum();

        // Return what is chosen by difficulty
        if total_fitness > 0.01 {
            return rng.choose_weighted(&by_difficulty, total_fitness);
        }

        // Return by random
        return rng.choose(&by_difficulty).unwrap().1.clone();
    }
}

struct Arch {
    areas: Vec<Area>,
}

struct Area {
    rooms: Vec<Room>,
}

impl Arch {
    fn new(areas: Vec<Area>) -> Arch {
        Arch { areas: areas }
    }
}

impl Area {
    fn new(rooms: Vec<Room>) -> Area {
        Area { rooms: rooms }
    }
}

trait DungeonRng<'a> {
    fn choose_many<'f, T, K: AsRef<T>>(&'a mut self, count: usize, pool: &'f [K]) -> Vec<T>
    where
        T: Clone;
    fn choose_weighted<T, K: AsRef<T>>(&mut self, pool: &[(f32, K)], total_weight: f32) -> T
    where
        T: Clone;
}

impl<'a> DungeonRng<'a> for StdRng {
    fn choose_many<'f, T, K: AsRef<T>>(&'a mut self, count: usize, pool: &'f [K]) -> Vec<T>
    where
        T: Clone,
    {
        let ref_pool: Vec<&T> = pool.iter().map(|x| x.as_ref()).collect();
        let mut ret = vec![];
        for _ in 0..count {
            let r: &T = *self.choose(ref_pool.as_slice()).unwrap();
            ret.push(r.clone())
        }
        ret
    }
    fn choose_weighted<T, K: AsRef<T>>(&mut self, pool: &[(f32, K)], total_weight: f32) -> T
    where
        T: Clone,
    {
        let pick = self.next_f32() * total_weight;
        let mut accumulator = 0.;
        for &(ref chance, ref item) in pool {
            accumulator += *chance;
            if pick < accumulator {
                return item.as_ref().clone();
            }
        }
        return self.choose(pool).unwrap().1.as_ref().clone();
    }
}
