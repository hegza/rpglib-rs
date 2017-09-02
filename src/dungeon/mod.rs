pub mod generator;

use monster::Monster;
use theme::Keyword;
use std::collections::HashMap;

pub struct Dungeon {
    pub rooms: Vec<Room>,
    /// Vector index is source, value[CompassPoint] is destination. Length is
    /// always equal to room count.
    passages: Vec<Passages>,
}

impl Dungeon {
    pub fn new(rooms: Vec<Room>) -> Dungeon {
        Dungeon {
            passages: vec![HashMap::new(); rooms.len()],
            rooms: rooms,
        }
    }
    pub fn first_room(&self) -> &Room {
        &self.rooms.first().expect("dungeon must have a room")
    }
    pub fn get_room_mut(&mut self, id: usize) -> &mut Room {
        &mut self.rooms[id]
    }
    pub fn get_room(&self, id: usize) -> &Room {
        &self.rooms[id]
    }
    pub fn get_adjacent(&self, source: usize, cp: CompassPoint) -> Option<&Room> {
        let room_id: Option<&usize> = self.passages[source].get(&cp);
        match room_id {
            None => None,
            Some(id) => Some(&self.rooms[*id]),
        }
    }
    pub fn create_passage(&mut self, source: usize, dir: CompassPoint, destination: usize) {
        {
            let rooms_passages: &mut Passages = &mut self.passages[source];
            rooms_passages.insert(dir, destination);
        }
        {
            let opposite_passages: &mut Passages = &mut self.passages[destination];
            opposite_passages.insert(dir.opposite(), source);
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Room {
    // TODO: temporary, replace with real content
    pub keyword: Keyword,
    // TODO: environment: Environment, modifier: RoomMod
    pub monster: Option<Monster>,
}

impl Room {
    pub fn new(kw: &Keyword, monster: Option<Monster>) -> Room {
        Room {
            keyword: kw.clone(),
            monster,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum CompassPoint {
    North,
    East,
    South,
    West,
}

impl CompassPoint {
    pub fn opposite(&self) -> CompassPoint {
        use CompassPoint::*;
        match self {
            &North => South,
            &East => West,
            &South => North,
            &West => East,
        }
    }
}

type Passages = HashMap<CompassPoint, usize>;
