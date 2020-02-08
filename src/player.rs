use crate::item::{Helmet, ChestArmor, Trousers, Boots, Weapon};
use crate::math::Vec2;

pub const WIDTH: u32 = 30;
pub const HEIGHT: u32 = 30;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub position: Vec2,
    pub equipment: Equipment,
    pub attributes: Attributes,
}

#[derive(Clone)]
pub struct Equipment {
    pub head: Option<Helmet>,
    pub chest: Option<ChestArmor>,
    pub legs: Option<Trousers>,
    pub boots: Option<Boots>,
    pub right_hand: Option<Weapon>,
    pub left_hand: Option<Weapon>,
}

impl Equipment {
    pub fn new() -> Self {
        Equipment {
            head: None,
            chest: None,
            legs: None,
            boots: None,
            right_hand: None,
            left_hand: None,
        }
    }
}

#[derive(Clone)]
pub struct Attributes {
    pub speed: i16,
    pub strength: u16,
    pub intellect: u16,
    pub charisma: u16,
    pub spirit: u16,
    pub agility: u16,
}
