use std::collections::HashSet;

use crate::input::CurrentInput;
use crate::inventory_ui::InventoryUi;
use crate::math::{vec2, Vec2};
use crate::msg::{Cmd, Msg, KeyInput, MouseButtonChange};
use crate::player::{self, Player, Equipment, Attributes};

pub struct Model {
    pub map: tiled::Map,
    pub camera_pos: Vec2,
    pub inventory: InventoryUi,
    pub player: Player,
    pub walkable_tiles: HashSet<u32>,
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

        let mut walkable_tiles = HashSet::new();
        for tileset in &map.tilesets {
            for tile in &tileset.tiles {
                if let Some(tile_type) = &tile.tile_type {
                    if tile_type == "walkable" {
                        walkable_tiles.insert(tileset.first_gid + tile.id);
                    }
                }
            }
        }

        Self {
            map,
            camera_pos: player.position,
            player,
            inventory: InventoryUi::new(50, 5, vec2(30., 30.), vec2(0., 0.)),
            walkable_tiles,
        }
    }

    pub fn update(self, msg: Msg) -> (Self, Vec<Cmd>) {
        match msg {
            Msg::Ignored => (self, vec!()),
            Msg::Input(input) => (self.handle_input(&input), vec!()),
            Msg::Tick(dt, keyboard_state) => (self.tick(dt, keyboard_state), vec!()),
            Msg::MouseMove{..} => (self, vec!()),
            Msg::MouseButtonChange(event) => self.on_mouse_button(event),
        }
    }

    pub fn tick(self, dt: f32, current_input: CurrentInput) -> Self {
        let old_player_pos = self.player.position;

        let mut new_player = self.player.tick(current_input, dt);

        let offsets_to_check = [
            vec2(-1., -1.), vec2(1., -1.), vec2(-1., 1.), vec2(1., 1.)
        ];
        let player_half_size = player::SIZE / 2;
        for corner_offset in &offsets_to_check {
            let pos = new_player.position + *corner_offset * player_half_size as f32;
            let tile_col = pos.x as usize / self.map.tile_width as usize;
            let tile_row = pos.y as usize / self.map.tile_height as usize;

            let tile = self.map.layers.last().unwrap().tiles[tile_row][tile_col];

            if !self.walkable_tiles.contains(&tile.gid) {
                let inside_tile_x = pos.x % self.map.tile_width as f32;
                let inside_tile_y = pos.y % self.map.tile_height as f32;

                let penetration_x = if corner_offset.x < 0. {
                    inside_tile_x - self.map.tile_width as f32
                } else {
                    inside_tile_x
                };
                let penetration_y = if corner_offset.y < 0. {
                    inside_tile_y - self.map.tile_height as f32
                } else {
                    inside_tile_y
                };

                if penetration_x.abs() < penetration_y.abs() {
                    new_player.position.x = old_player_pos.x;
                } else {
                    new_player.position.y = old_player_pos.y;
                }
            }
        }

        let new_camera_pos = new_player.position;

        Self {
            player: new_player,
            camera_pos: new_camera_pos,
            .. self
        }
    }

    pub fn handle_input(self, input: &KeyInput) -> Self {
        self
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
