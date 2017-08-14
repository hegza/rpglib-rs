use super::item::*;

pub trait YieldReward<'b> {
    fn reward<'a>(&'a self) -> &Option<&'a Reward<'b>>;
}

pub enum Reward<'a> {
    Item(&'a EquipItem),
}
