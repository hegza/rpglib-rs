extern crate inflector;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;
extern crate try_from;
extern crate range;
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate rustache;

// Keep the #[macro use] utils first
#[macro_use]
pub mod utils;
pub mod item;
pub mod combat;
pub mod character;
pub mod display;
pub mod monster;
pub mod theme;
pub mod dungeon;

#[cfg(test)]
mod tests;

pub use item::*;
pub use combat::*;
pub use character::*;
pub use utils::*;
pub use theme::*;
pub use display::*;
pub use monster::*;
pub use dungeon::*;
