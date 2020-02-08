use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use crate::assets::Assets;
use crate::item::{Helmet, ChestArmor, Trousers, Boots, Weapon};
use crate::math::{vec2, Vec2};
use crate::input::CurrentInput;

pub const WIDTH: u32 = 30;
pub const HEIGHT: u32 = 30;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub position: Vec2,
    pub equipment: Equipment,
    pub attributes: Attributes,
}

impl Player {
    pub fn tick(self, input: CurrentInput, dt: f32) -> Self {
        let mut dx = 0.;
        dx += if input.right {1.} else {0.} * 100. * dt;
        dx += if input.left {-1.} else {0.} * 100. * dt;

        let mut dy = 0.;
        dy += if input.down {1.} else {0.} * 100. * dt;
        dy += if input.up {-1.} else {0.} * 100. * dt;

        let new_position = self.position + vec2(dx, dy);
        Self { position: new_position, .. self}
    }

    pub fn view<T: RenderTarget>(&self, canvas: &mut Canvas<T>, assets: &Assets, cam_offset: Vec2) {
        let player_screen_pos = self.position + cam_offset;
        let player_rect = Rect::new(
            player_screen_pos.x as i32 - WIDTH as i32 / 2,
            player_screen_pos.y as i32 - HEIGHT as i32 / 2,
            WIDTH,
            HEIGHT,
        );
        canvas.copy(&assets.player_texture, None, player_rect).unwrap();
    }
}

#[derive(Clone)]
pub struct Equipment {
    pub head: Option<Helmet>,
    pub chest: Option<ChestArmor>,
    pub legs: Option<Trousers>,
    pub boots: Option<Boots>,
    pub right_hand: Option<Weapon>,
    pub left_hand: Option<Weapon>,
}

impl Equipment {
    pub fn new() -> Self {
        Equipment {
            head: None,
            chest: None,
            legs: None,
            boots: None,
            right_hand: None,
            left_hand: None,
        }
    }
}

#[derive(Clone)]
pub struct Attributes {
    pub speed: i16,
    pub strength: u16,
    pub intellect: u16,
    pub charisma: u16,
    pub spirit: u16,
    pub agility: u16,
}
