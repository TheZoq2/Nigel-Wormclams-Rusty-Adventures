use std::collections::HashMap;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Assets<'r> {
    // tilesets are matched by their .first_gid
    pub tileset_textures: HashMap<u32, Texture<'r>>,
    pub player_texture: Texture<'r>,
}

impl<'r> Assets<'r> {
    pub fn new(texture_creator: &'r TextureCreator<WindowContext>, map: &tiled::Map) -> Self {
        let tileset_textures = map.tilesets.iter().map(|tileset| {
            let tileset_image_file = &tileset.images[0].source;
            let tex = texture_creator.load_texture(
                String::from("assets/maps/") + tileset_image_file
            ).unwrap();
            (tileset.first_gid, tex)
        }).collect();

        let player_texture = texture_creator.load_texture(
            "assets/images/player.png"
        ).unwrap();

        Assets {
            tileset_textures,
            player_texture,
        }
    }
}
