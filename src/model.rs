use sdl2::keyboard::Keycode;

use crate::inventory_ui::InventoryUi;
use crate::player::{Player, Equipment, Attributes};
use crate::math::{Vec2, vec2};
use crate::msg::{Cmd, Msg, KeyInput, MouseButtonChange};

#[derive(Default)]
pub struct CurrentInput {
    left: bool,
    right: bool,
}
impl CurrentInput {
    pub fn update(self, input: &KeyInput) -> Self {
        match input {
            KeyInput::KeyDown(Keycode::Left) => {
                Self{left: true, .. self}
            }
            KeyInput::KeyDown(Keycode::Right) => {
                Self{right: true, .. self}
            }
            KeyInput::KeyUp(Keycode::Left) => {
                Self{left: false, .. self}
            }
            KeyInput::KeyUp(Keycode::Right) => {
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
            player,
            inventory: InventoryUi::new(50, 5, vec2(30., 30.), vec2(0., 0.))
        }
    }

    pub fn update(self, msg: Msg) -> (Self, Vec<Cmd>) {
        match msg {
            Msg::Ignored => (self, vec!()),
            Msg::Input(input) => (self.handle_input(&input), vec!()),
            Msg::Tick(dt) => (self.tick(dt), vec!()),
            Msg::MouseMove{..} => (self, vec!()),
            Msg::MouseButtonChange(event) => self.on_mouse_button(event),
        }
    }

    pub fn tick(self, dt: f32) -> Self {
        let mut new_pos = self.pos;
        new_pos += if self.input.right {1.} else {0.} * 100. * dt;
        new_pos += if self.input.left {-1.} else {0.} * 100. * dt;
        Self {pos: new_pos, .. self}
    }

    pub fn handle_input(self, input: &KeyInput) -> Self {
        Self{input: self.input.update(input), .. self}
    }

    pub fn on_mouse_move(self, position: Vec2) -> (Self, Vec<Cmd>) {
        unimplemented!()
    }
    pub fn on_mouse_button(self, event: MouseButtonChange) -> (Self, Vec<Cmd>) {
        if self.inventory.is_on_inventory(event.pos) {
            let (inventory, cmds) = self.inventory.on_mouse_button(event);
            (Self{inventory, .. self}, cmds)
        }
        else {
            (self, vec!())
        }
    }
}
