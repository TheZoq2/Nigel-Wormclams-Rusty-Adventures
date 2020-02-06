use sdl2::keyboard::Keycode;

pub enum Input {
    KeyDown(Keycode),
    KeyUp(Keycode),
    MouseMove(f32, f32),
    LeftClick(f32, f32),
    RightClick(f32, f32),
}
