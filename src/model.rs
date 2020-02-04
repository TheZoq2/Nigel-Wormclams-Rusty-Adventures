use sdl2::keyboard::Keycode;

use crate::input::{Input};

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
    pub pos: f32,
}


impl Model {
    pub fn init() -> Self {
        Self {
            input: CurrentInput::default(),
            pos: 0.,
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
