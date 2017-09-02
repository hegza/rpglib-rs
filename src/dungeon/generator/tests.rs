use super::super::*;
use range::Range;
use dungeon::generator::*;
use std::cmp::{min, max, Ordering};
use std::f32;

// Strolneg, M, god of not ingesting poisonous plants and avoiding poison in general, paranoid
// Zarad-dul, F, goddess of creating holes in people, trees, and the ground, etc; consumed with an all-encompassing rage
// Iahu, M, god of pointy objects, perpetually lost (so don't pray to him unless you have a map) and befuddled
// Eregek, F, goddess of strangulation, careless (and might strangle you accidentally, should you ask for help)
// Gzolneb, M, god of death by crushing, deeply insecure and prone to overkilling everything ever
// Urra, F, goddess of ducking and anxiety attacks, fidgety, twitchy, and high-strung
lazy_static! {
    static ref SEED: Vec<usize> = vec![1, 2, 3, 4];
    static ref MONSTER_POOL: Vec<Monster> = vec![
        MonsterBuilder::new("goblin", 1, 3).difficulty(1).spawn()
    ];
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
                           Range::new(2, 2),
                           Range::new(4, 2));
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
    let rooms = vec![Room::new(&Keyword { id: "A".to_string() }, None),
                     Room::new(&Keyword { id: "B".to_string() }, None)];
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

#[test]
fn theme_calculation_works() {
    let monster_1 = MonsterBuilder::new("goblin", 1, 1).difficulty(10).keywords(vec!["goblin"].as_slice()).spawn();
    let monster_2 = MonsterBuilder::new("demon", 1, 1).difficulty(10).keywords(vec!["demon"].as_slice()).spawn();
    let monster_3 = MonsterBuilder::new("demon-goblin", 1, 1).difficulty(10).keywords(vec!["goblin", "demon"].as_slice()).spawn();

    let theme_1: Vec<Keyword> = vec!["goblin".into()];
    let theme_2: Vec<Keyword> = vec!["demon".into()];
    let theme_3: Vec<Keyword> = vec!["goblin".into(), "demon".into()];

    // Act
    let m1_in_t1 = evaluate_theme(&monster_1, theme_1.as_slice());
    let m2_in_t1 = evaluate_theme(&monster_2, theme_1.as_slice());
    let m3_in_t1 = evaluate_theme(&monster_3, theme_1.as_slice());

    let m1_in_t2 = evaluate_theme(&monster_1, theme_2.as_slice());
    let m2_in_t2 = evaluate_theme(&monster_2, theme_2.as_slice());
    let m3_in_t2 = evaluate_theme(&monster_3, theme_2.as_slice());

    let m1_in_t3 = evaluate_theme(&monster_1, theme_3.as_slice());
    let m2_in_t3 = evaluate_theme(&monster_2, theme_3.as_slice());
    let m3_in_t3 = evaluate_theme(&monster_3, theme_3.as_slice());
    const MAX_DELTA: f32 = 0.01;

    // Assert
    assert!( (m1_in_t1 - 1.0).abs() < MAX_DELTA );
    assert!( (m2_in_t1 - 0.0).abs() < MAX_DELTA );
    assert!( (m3_in_t1 - 1.0).abs() < MAX_DELTA );

    assert!( (m1_in_t2 - 0.0).abs() < MAX_DELTA );
    assert!( (m2_in_t2 - 1.0).abs() < MAX_DELTA );
    assert!( (m3_in_t2 - 1.0).abs() < MAX_DELTA );

    assert!( (m1_in_t3 - 0.5).abs() < MAX_DELTA );
    assert!( (m2_in_t3 - 0.5).abs() < MAX_DELTA );
    assert!( (m3_in_t3 - 1.0).abs() < MAX_DELTA );
}

#[test]
fn difficulty_is_normalized() {
    let goblin = MonsterBuilder::new("goblin", 1, 3).difficulty(1).spawn();
    let demon = MonsterBuilder::new("demon", 15, 40).difficulty(10).spawn();
    assert_eq!(goblin.normalized_difficulty(), 0.1);
    assert_eq!(demon.normalized_difficulty(), 1.);
}
