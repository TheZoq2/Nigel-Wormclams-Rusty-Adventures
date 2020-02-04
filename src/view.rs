use sdl2::render::{Canvas, RenderTarget};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::model::Model;

pub fn view<T: RenderTarget>(model: &Model, canvas: &mut Canvas<T>) {
    canvas.set_draw_color(Color::RGB(0, 50, 80));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 80));

    canvas.draw_rect(Rect::new(model.pos as i32, 0, 10, 10));
}
