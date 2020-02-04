use sdl2::keyboard::Keycode;

pub enum Input {
    KeyDown(Keycode),
    KeyUp(Keycode)
}
