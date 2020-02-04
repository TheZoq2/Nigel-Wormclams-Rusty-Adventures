pub enum Key {
    Right,
    Left,
}

pub enum Input {
    KeyDown(Key),
    KeyUp(Key)
}
