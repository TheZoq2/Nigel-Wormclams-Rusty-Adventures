use sdl2::keyboard::Keycode;

use crate::msg::KeyInput;

#[derive(Default, Clone, Copy)]
pub struct CurrentInput {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}
impl CurrentInput {
    pub fn update(self, input: &KeyInput) -> Self {
        match input {
            KeyInput::KeyDown(kc) => {
                self.set_key(*kc, true)
            }
            KeyInput::KeyUp(kc) => {
                self.set_key(*kc, false)
            }
            _ => {self}
        }
    }

    fn set_key(self, keycode: Keycode, state: bool) -> Self {
        match keycode {
            Keycode::Left => {
                Self{left: state, .. self}
            }
            Keycode::Right => {
                Self{right: state, .. self}
            }
            Keycode::Up => {
                Self{up: state, .. self}
            }
            Keycode::Down => {
                Self{down: state, .. self}
            }
            _ => {self}
        }
    }
}
