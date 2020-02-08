use sdl2::keyboard::Keycode;

use crate::input::{Input};
use crate::inventory_ui::InventoryUi;
use crate::math::vec2;
use crate::player::{Player, Equipment, Attributes};

#[derive(Default)]
pub struct CurrentInput {
    left: bool,
    right: bool,
}
impl CurrentInput {
    pub fn update(self, input: &Input) -> Self {
        match input {
            Input::KeyDown(Keycode::Left) => {
                Self{left: true, .. self}
            }
            Input::KeyDown(Keycode::Right) => {
                Self{right: true, .. self}
            }
            Input::KeyUp(Keycode::Left) => {
                Self{left: false, .. self}
            }
            Input::KeyUp(Keycode::Right) => {
                Self{right: false, .. self}
            }
            _ => {self}
        }
    }
}

pub struct Model {
    pub input: CurrentInput,
    pub map: tiled::Map,
    pub pos: f32,
    pub inventory: InventoryUi,
    pub player: Player,
}


impl Model {
    pub fn init(map: tiled::Map) -> Self {
        let player = Player {
            name: "Nigel Wormclam".to_string(),
            position: vec2(832., 1184.),
            equipment: Equipment::new(),
            attributes: Attributes {
                strength: 30,
                speed: 200,
                agility: 24,
                intellect: 14,
                charisma: 9,
                spirit: 16,
            },
        };
        Self {
            input: CurrentInput::default(),
            map,
            pos: 0.,
            inventory: InventoryUi::new(50, 5, vec2(30., 30.)),
            player
        }
    }

    pub fn tick(self, dt: f32) -> Self {
        let mut new_pos = self.pos;
        new_pos += if self.input.right {1.} else {0.} * 100. * dt;
        new_pos += if self.input.left {-1.} else {0.} * 100. * dt;
        Self {pos: new_pos, .. self}
    }

    pub fn handle_input(self, input: &Input) -> Self {
        Self{input: self.input.update(input), .. self}
    }
}
