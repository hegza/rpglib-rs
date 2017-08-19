use super::item::*;

pub trait YieldReward {
    fn reward(&self) -> Option<Reward>;
}

pub enum Reward {
    Item(Item),
}
