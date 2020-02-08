use std::time::Instant;
use std::path::Path;

use sdl2::event::Event;
use sdl2::pixels::Color;

mod assets;
mod input;
mod inventory;
mod inventory_ui;
mod item;
mod math;
mod model;
mod msg;
mod player;
mod view;

use assets::Assets;
use msg::{Cmd, Msg};
use model::Model;
use input::{Input};

fn update(msg: Msg, model: Model) -> (Model, Vec<Cmd>) {
    match msg {
        Msg::Input(input) => (model.handle_input(&input), vec!()),
        Msg::Tick(dt) => (model.tick(dt), vec!()),
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Nigel Wormclams Magic Adventure", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
 
    canvas.set_draw_color(Color::RGB(0, 50, 80));
    canvas.clear();
    canvas.present();

    let map_location = "assets/maps/overworld.tmx";
    let map = tiled::parse_file(&Path::new(map_location))
        .expect("Could not parse map file assets/maps/overworld.tmx");

    let texture_creator = canvas.texture_creator();
    let assets = Assets::new(&texture_creator, &map);

    let mut model = Model::init(map);

    let dt = 1. / 60.;
    let mut now = Instant::now();
    let mut accumulated_time = 0.;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let mut msgs = vec!();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                }
                Event::MouseMotion{x, y, ..} => {
                    msgs.push(Msg::Input(Input::MouseMove(x as f32, y as f32)))
                }
                Event::MouseButtonDown{mouse_btn, x, y, ..} => {
                    let (x, y) = (x as f32, y as f32);
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            msgs.push(Msg::Input(Input::LeftClick(x, y)))
                        }
                        sdl2::mouse::MouseButton::Right => {
                            msgs.push(Msg::Input(Input::RightClick(x, y)))
                        }
                        _ => {}
                    }
                }
                Event::KeyDown { keycode: Some(kc), .. } => {
                    msgs.push(Msg::Input(Input::KeyDown(kc)))
                }
                Event::KeyUp { keycode: Some(kc), .. } => {
                    msgs.push(Msg::Input(Input::KeyUp(kc)))
                }
                _ => {}
            }
        }

        let new_now = Instant::now();
        accumulated_time += new_now.duration_since(now).as_secs_f32();
        now = new_now;

        while accumulated_time >= dt {
            accumulated_time -= dt;

            msgs.push(Msg::Tick(dt));
        }
        while let Some(msg) = msgs.pop() {
            let (new_model, _new_cmds) = update(msg, model);
            model = new_model;
        }

        view::view(&model, &mut canvas, &assets);

        canvas.present();
    }
}
