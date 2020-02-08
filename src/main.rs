use std::time::Instant;
use std::path::Path;

use sdl2::event::Event;
use sdl2::pixels::Color;

mod assets;
mod msg;
mod model;
mod view;
mod input;
mod inventory;
mod inventory_ui;
mod item;
mod math;
mod player;

use assets::Assets;
use msg::{Cmd, Msg, KeyInput, MouseButton, MouseButtonChange};
use model::Model;
use math::vec2;

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
                Event::MouseMotion{x, y, mousestate: state, ..} => {
                    msgs.push(Msg::MouseMove{
                        pos: vec2(x as f32, y as f32),
                        state,
                    })
                }
                Event::MouseButtonDown{mouse_btn, x, y, ..} => {
                    let pos = vec2(x as f32, y as f32);
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            msgs.push(Msg::MouseButtonChange(MouseButtonChange{
                                pos,
                                button: MouseButton::Left,
                                pressed: true
                            }))
                        }
                        sdl2::mouse::MouseButton::Right => {
                            msgs.push(Msg::MouseButtonChange(MouseButtonChange{
                                pos,
                                button: MouseButton::Right,
                                pressed: true
                            }))
                        }
                        _ => {}
                    }
                }
                Event::KeyDown { keycode: Some(kc), .. } => {
                    msgs.push(Msg::Input(KeyInput::KeyDown(kc)))
                }
                Event::KeyUp { keycode: Some(kc), .. } => {
                    msgs.push(Msg::Input(KeyInput::KeyUp(kc)))
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
            let (new_model, _new_cmds) = model.update(msg);
            model = new_model;
        }

        view::view(&model, &mut canvas, &assets);

        canvas.present();
    }
}
