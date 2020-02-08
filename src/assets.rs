use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Assets<'r> {
    pub tileset_texture: Texture<'r>,
    pub player_texture: Texture<'r>,
}

impl<'r> Assets<'r> {
    pub fn new(texture_creator: &'r TextureCreator<WindowContext>, map: &tiled::Map) -> Self {
        // TODO: Support multiple tilesets and images
        let tileset = &map.tilesets[0];
        let tileset_image_file = &tileset.images[0].source;
        let tileset_texture = texture_creator.load_texture(
            String::from("assets/maps/") + tileset_image_file
        ).unwrap();

        let player_texture = texture_creator.load_texture(
            "assets/images/player.png"
        ).unwrap();

        Assets {
            tileset_texture,
            player_texture,
        }
    }
}
