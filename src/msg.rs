use crate::input::Input;

pub enum Cmd {
    Loopback(Msg),
}

pub enum Msg {
    Input(Input),
    Tick(f32),
}
