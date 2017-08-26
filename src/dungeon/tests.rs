use super::super::*;
use range::Range;
use dungeon::generator::*;

// Strolneg, M, god of not ingesting poisonous plants and avoiding poison in general, paranoid
// Zarad-dul, F, goddess of creating holes in people, trees, and the ground, etc; consumed with an all-encompassing rage
// Iahu, M, god of pointy objects, perpetually lost (so don't pray to him unless you have a map) and befuddled
// Eregek, F, goddess of strangulation, careless (and might strangle you accidentally, should you ask for help)
// Gzolneb, M, god of death by crushing, deeply insecure and prone to overkilling everything ever
// Urra, F, goddess of ducking and anxiety attacks, fidgety, twitchy, and high-strung
lazy_static! {
    static ref SEED: Vec<usize> = vec![1, 2, 3, 4];
    static ref MONSTER_POOL: Vec<Monster> = vec![MonsterBuilder::new("goblin", 1, 3).difficulty(1).spawn()];
    static ref TEMPLATE_MONSTER_POOL: Vec<TemplateMonster> =
        vec![TemplateMonster::new(MonsterBuilder::new("servant_of_{}", 1, 3).difficulty(2).spawn(), hashmap!("strolneg".into() => vec![]))];
    static ref THEME_KEYWORD_POOL: Vec<Keyword> = vec![
        // Gods
        "strolneg", "zarad-dul", "iahu", "eregek", "gzolneb", "urra",
        // Creature types
        "spider", "goblin", "elf",
        // Magical effects
        "giant"]
        .iter().map(|x: &&str| Keyword::from(x.to_string().clone())).collect();
    static ref GENERATOR: Generator<'static> = Generator::new(MONSTER_POOL.as_slice(),
                           TEMPLATE_MONSTER_POOL.as_slice(),
                           THEME_KEYWORD_POOL.as_slice(),
                           10,
                           5,
                           3,
                           3,
                           Range::new(2, 4),
                           Range::new(4, 6));
}

/// Verify that dungeon is generated
#[test]
fn is_generated() {
    // Arrange

    // Act
    let dungeon = GENERATOR.generate(&SEED.as_slice());

    // Assert
    assert!(dungeon.rooms.len() != 0);
}

#[test]
fn correct_main_path_length() {
    // Arrange
    let arch_count = 2;
    let num_areas_in_arch = 2;
    let num_main_rooms_in_area = 5;
    let g = Generator::new(MONSTER_POOL.as_slice(),
                           TEMPLATE_MONSTER_POOL.as_slice(),
                           THEME_KEYWORD_POOL.as_slice(),
                           8,
                           5,
                           3,
                           arch_count,
                           Range::new(num_areas_in_arch, 1),
                           Range::new(num_main_rooms_in_area, 1));

    // Act
    let dungeon = g.generate(&SEED.as_slice());

    // Assert
    let expected_area_count = arch_count * num_areas_in_arch * num_main_rooms_in_area;
    assert_eq!(dungeon.rooms.len(), expected_area_count);
}

#[test]
fn passage_works() {
    let rooms = vec![Room::new(&Keyword { id: "A".to_string() }),
                     Room::new(&Keyword { id: "B".to_string() })];
    let mut dungeon = Dungeon::new(rooms);

    dungeon.create_passage(0, CompassPoint::East, 1);

    let room_a = dungeon.get_room(0);
    let room_b = dungeon.get_room(1);
    assert_eq!(dungeon.get_adjacent(0, CompassPoint::East).unwrap().keyword,
               room_b.keyword);
    assert_eq!(dungeon.get_adjacent(1, CompassPoint::West).unwrap().keyword,
               room_a.keyword);
}

// Verify that dungeons generated with a specific seed are always identical
#[test]
fn dungeon_is_deterministic() {
    let dungeon_a = GENERATOR.generate(&SEED);
    let dungeon_b = GENERATOR.generate(&SEED);

    assert_eq!(dungeon_a.rooms.len(), dungeon_b.rooms.len());
    for r in 0..dungeon_a.rooms.len() {
        assert_eq!(dungeon_a.get_room(r).keyword, dungeon_b.get_room(r).keyword)
    }
}

// TODO: verify that dungeons respect their invariant arguments
/*
 * TODO: verify that sub-dungeons always only contain theme keywords of their
 * super-dungeons
 */
