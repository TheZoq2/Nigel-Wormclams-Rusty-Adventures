use crate::math::Vec2;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;

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
    Tick(f32),
    MouseMove{pos: Vec2, state: MouseState},
    MouseButtonChange(MouseButtonChange),
}
