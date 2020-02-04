use crate::input::{Input, Key};

#[derive(Default)]
pub struct CurrentInput {
    left: bool,
    right: bool,
}
impl CurrentInput {
    pub fn update(self, input: &Input) -> Self {
        match input {
            Input::KeyDown(Key::Left) => {
                Self{left: true, .. self}
            }
            Input::KeyDown(Key::Right) => {
                Self{right: true, .. self}
            }
            Input::KeyUp(Key::Left) => {
                Self{left: false, .. self}
            }
            Input::KeyUp(Key::Right) => {
                Self{right: false, .. self}
            }
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
