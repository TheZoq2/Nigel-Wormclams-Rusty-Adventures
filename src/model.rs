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
        let old_pos = self.player.position;
        let new_player = self.player.tick(current_input, dt);

        let player_half_size = player::SIZE as f32 / 2.;

        let map = &self.map;
        let tile_at = |pos: Vec2| -> tiled::LayerTile {
            let tile_col = pos.x as usize / map.tile_width as usize;
            let tile_row = pos.y as usize / map.tile_height as usize;

            map.layers.last().unwrap().tiles[tile_row][tile_col]
        };

        let walkable_tiles = &self.walkable_tiles;
        let collision_at = |pos: Vec2| -> bool {
            let tile = tile_at(pos);
            !walkable_tiles.contains(&tile.gid)
        };

        let mut new_pos = new_player.position;

        // Check x axis
        {
            let top = old_pos.y -player_half_size;
            let bottom = old_pos.y + player_half_size;
            let left = new_pos.x -player_half_size;
            let right = new_pos.x + player_half_size;

            let top_left_collision = collision_at(vec2(left, top));
            let top_right_collision = collision_at(vec2(right, top));
            let bottom_left_collision = collision_at(vec2(left, bottom));
            let bottom_right_collision = collision_at(vec2(right, bottom));

            let inside_tile_left = left % self.map.tile_width as f32;
            let penetration_left = self.map.tile_width as f32 - inside_tile_left;
            let penetration_right = right % self.map.tile_width as f32 + 1.;

            // Left side
            if top_left_collision || bottom_left_collision {
                new_pos.x += penetration_left;
            }

            // Right side
            if top_right_collision || bottom_right_collision {
                new_pos.x -= penetration_right;
            }
        }

        // Check y axis
        {
            let top = new_pos.y -player_half_size;
            let bottom = new_pos.y + player_half_size;
            let left = new_pos.x -player_half_size;
            let right = new_pos.x + player_half_size;

            let top_left_collision = collision_at(vec2(left, top));
            let top_right_collision = collision_at(vec2(right, top));
            let bottom_left_collision = collision_at(vec2(left, bottom));
            let bottom_right_collision = collision_at(vec2(right, bottom));

            let inside_tile_top = top % self.map.tile_height as f32;
            let penetration_top = self.map.tile_height as f32 - inside_tile_top;
            let penetration_bottom = bottom % self.map.tile_height as f32 + 1.;

            // Top side
            if top_left_collision || top_right_collision {
                new_pos.y += penetration_top;
            }

            // Bottom side
            if bottom_left_collision || bottom_right_collision {
                new_pos.y -= penetration_bottom;
            }
        }

        let new_camera_pos = new_player.position;

        Self {
            player: Player {
                position: new_pos,
                .. new_player
            },
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
