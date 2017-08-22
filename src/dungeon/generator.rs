use theme::Keyword;
use rand::{Rng, SeedableRng, StdRng};
use range::Range;
use monster::*;
use super::*;

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
}

impl<'a> Generator<'a> {
    pub fn new(monster_pool: &'a [Monster],
               template_monster_pool: &'a [TemplateMonster],
               theme_keyword_pool: &'a [Keyword],
               dungeon_keyword_count: usize,
               arch_keyword_count: usize,
               area_keyword_count: usize,
               arch_count: usize,
               num_areas_in_arch: Range,
               num_main_rooms_in_area: Range)
               -> Generator<'a> {
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

        let mut dungeon = Dungeon::new(rooms);
        // Generate paths
        for i in 0..dungeon.rooms.len() - 1 {
            dungeon.create_passage(i, CompassPoint::East, i + 1);
        }

        dungeon
    }
    fn generate_arches<'b>(&'a self,
                           keyword_pool: &'b [Keyword],
                           mut rng: &'b mut StdRng)
                           -> Vec<Arch> {
        let g = self;

        let mut arches = Vec::with_capacity(g.arch_count);
        for _ in 0..g.arch_count {
            let arch_keywords = rng.choose_many(g.arch_keyword_count, keyword_pool);

            // Create the arch and areas
            let r = g.num_areas_in_arch;
            let num_areas_in_arch = rng.gen_range(r.offset, r.offset + r.length);
            let arch =
                Arch::new(arch_keywords.as_slice(),
                          g.generate_areas(num_areas_in_arch, arch_keywords.as_slice(), &mut rng));
            arches.push(arch)
        }
        arches
    }
    fn generate_areas<'b>(&'a self,
                          count: usize,
                          keyword_pool: &'b [Keyword],
                          mut rng: &'b mut StdRng)
                          -> Vec<Area> {
        let g = self;

        let mut areas = Vec::with_capacity(count);
        for _ in 0..count {
            let area_keywords = rng.choose_many(g.area_keyword_count, keyword_pool);

            // Create the area and rooms
            let r = g.num_main_rooms_in_area;
            let num_main_rooms_in_area = rng.gen_range(r.offset, r.offset + r.length);
            let area = Area::new(area_keywords.as_slice(),
                                 g.generate_rooms(num_main_rooms_in_area,
                                                  area_keywords.as_slice(),
                                                  &mut rng));
            areas.push(area);
        }
        areas
    }
    fn generate_rooms<'b>(&'a self,
                          count: usize,
                          keyword_pool: &'b [Keyword],
                          rng: &'b mut StdRng)
                          -> Vec<Room> {
        let g = self;

        let mut rooms = Vec::with_capacity(count);
        for _ in 0..count {
            let keyword = rng.choose(keyword_pool).unwrap();

            // Create the area and rooms
            rooms.push(Room::new(keyword));
        }
        rooms
    }
}

struct Arch {
    theme: Vec<Keyword>,
    areas: Vec<Area>,
}

struct Area {
    theme: Vec<Keyword>,
    rooms: Vec<Room>,
}

impl Arch {
    fn new(keywords: &[Keyword], areas: Vec<Area>) -> Arch {
        Arch {
            theme: keywords.to_vec(),
            areas: areas,
        }
    }
}

impl Area {
    fn new(keywords: &[Keyword], rooms: Vec<Room>) -> Area {
        Area {
            theme: keywords.to_vec(),
            rooms: rooms,
        }
    }
}

trait DungeonRng<'a> {
    fn choose_many<'f, T, K: AsRef<T>>(&'a mut self, count: usize, pool: &'f [K]) -> Vec<T>
        where T: Clone;
}

impl<'a> DungeonRng<'a> for StdRng {
    fn choose_many<'f, T, K: AsRef<T>>(&'a mut self, count: usize, pool: &'f [K]) -> Vec<T>
        where T: Clone
    {
        let ref_pool: Vec<&T> = pool.iter().map(|x| x.as_ref()).collect();
        let mut ret = vec![];
        for _ in 0..count {
            let r: &T = *self.choose(ref_pool.as_slice()).unwrap();
            ret.push(r.clone())
        }
        ret
    }
}
