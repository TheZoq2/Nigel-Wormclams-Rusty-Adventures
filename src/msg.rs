use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;

use crate::input::CurrentInput;
use crate::math::Vec2;

#[derive(Debug, Clone)]
pub enum KeyInput {
    KeyDown(Keycode),
    KeyUp(Keycode),
}

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Right
}
#[derive(Debug, Clone)]
pub struct MouseButtonChange {
    pub pos: Vec2,
    pub pressed: bool,
    pub button: MouseButton
}

pub enum Cmd {
    Loopback(Msg),
}

pub enum Msg {
    Ignored,
    Input(KeyInput),
    Tick(f32, CurrentInput),
    MouseMove{pos: Vec2, state: MouseState},
    MouseButtonChange(MouseButtonChange),
}
