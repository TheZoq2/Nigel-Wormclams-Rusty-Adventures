use sdl2::render::{Canvas, RenderTarget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::assets::Assets;
use crate::math::vec2;
use crate::model::Model;

pub fn view<T: RenderTarget>(model: &Model, canvas: &mut Canvas<T>, assets: &Assets) {
    canvas.set_draw_color(Color::RGB(0, 50, 80));
    canvas.clear();

    let (screen_w, screen_h) = canvas.output_size().unwrap();
    let screen_center = vec2(
        screen_w as f32 * 0.5,
        screen_h as f32 * 0.5,
    );
    let cam_offset = screen_center - model.camera_pos;

    let tile_width = model.map.tile_width;
    let tile_height = model.map.tile_height;

    for layer in &model.map.layers {
        for (row_i, row_tiles) in layer.tiles.iter().enumerate() {
            for (col_i, tile) in row_tiles.iter().enumerate() {
                if let Some(tileset) = model.map.get_tileset_by_gid(tile.gid) {
                    let tileset_width = tileset.images[0].width as u32 / tile_width;

                    let tile_id = tile.gid - tileset.first_gid;
                    let tile_x = tile_id % tileset_width;
                    let tile_y = tile_id / tileset_width;
                    let src = Rect::new(
                        (tile_x * tile_width) as i32,
                        (tile_y * tile_height) as i32,
                        tile_width,
                        tile_height
                    );
                    let dest = Rect::new(
                        col_i as i32 * tile_width as i32 + cam_offset.x as i32,
                        row_i as i32 * tile_height as i32 + cam_offset.y as i32,
                        tile_width,
                        tile_height
                    );

                    let tex = &assets.tileset_textures[&tileset.first_gid];
                    canvas.copy(tex, src, dest).unwrap();
                }
            }
        }
    }

    model.inventory.draw(canvas);

    model.player.view(canvas, assets, cam_offset);
}
