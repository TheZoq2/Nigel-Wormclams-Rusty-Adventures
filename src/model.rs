use crate::input::CurrentInput;
use crate::inventory_ui::InventoryUi;
use crate::math::{vec2, Vec2};
use crate::player::{Player, Equipment, Attributes};
use crate::msg::{Cmd, Msg, KeyInput, MouseButtonChange};

pub struct Model {
    pub input: CurrentInput,
    pub map: tiled::Map,
    pub camera_pos: Vec2,
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
            camera_pos: player.position,
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
        let new_player = self.player.tick(self.input, dt);
        let new_camera_pos = new_player.position;
        Self {player: new_player, camera_pos: new_camera_pos, .. self}
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
