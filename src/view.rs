use sdl2::render::{Canvas, RenderTarget, Texture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::model::Model;

pub fn view<T: RenderTarget>(model: &Model, canvas: &mut Canvas<T>, tileset_texture: &Texture) {
    canvas.set_draw_color(Color::RGB(0, 50, 80));
    canvas.clear();

    let tile_width = model.map.tile_width;
    let tile_height = model.map.tile_height;

    // TODO: Calculate which tileset and image we should use
    let tileset_width = model.map.tilesets[0].images[0].width as u32 / tile_width;

    for layer in &model.map.layers {
        for (row_i, row_tiles) in layer.tiles.iter().enumerate() {
            for (col_i, tile) in row_tiles.iter().enumerate() {
                if *tile == 0 {
                    continue;
                }

                let tile = tile - 1;
                let tile_x = tile % tileset_width;
                let tile_y = tile / tileset_width;
                let src = Rect::new(
                    (tile_x * tile_width) as i32,
                    (tile_y * tile_height) as i32,
                    tile_width,
                    tile_height
                );
                let dest = Rect::new(
                    col_i as i32 * tile_width as i32,
                    row_i as i32 * tile_height as i32,
                    tile_width,
                    tile_height
                );

                canvas.copy(tileset_texture, src, dest).unwrap();
            }
        }
    }

    canvas.set_draw_color(Color::RGB(255, 255, 80));
    canvas.draw_rect(Rect::new(model.pos as i32, 0, 10, 10)).unwrap();
}
