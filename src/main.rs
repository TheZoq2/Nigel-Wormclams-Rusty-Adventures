use std::path::Path;
use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
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
use input::CurrentInput;
use math::vec2;
use model::Model;
use msg::{Cmd, Msg, KeyInput, MouseButton, MouseButtonChange};

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

    macro_rules! update_model {
        ($msg:expr) => {
            let (new_model, _new_cmds) = model.update($msg);
            model = new_model;
        }
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                }
                Event::MouseMotion{x, y, mousestate: state, ..} => {
                    update_model!(Msg::MouseMove{
                        pos: vec2(x as f32, y as f32),
                        state,
                    });
                }
                Event::MouseButtonDown{mouse_btn, x, y, ..} => {
                    let pos = vec2(x as f32, y as f32);
                    match mouse_btn {
                        sdl2::mouse::MouseButton::Left => {
                            update_model!(Msg::MouseButtonChange(MouseButtonChange{
                                pos,
                                button: MouseButton::Left,
                                pressed: true
                            }));
                        }
                        sdl2::mouse::MouseButton::Right => {
                            update_model!(Msg::MouseButtonChange(MouseButtonChange{
                                pos,
                                button: MouseButton::Right,
                                pressed: true
                            }));
                        }
                        _ => {}
                    }
                }
                Event::KeyDown { keycode: Some(kc), .. } => {
                    update_model!(Msg::Input(KeyInput::KeyDown(kc)));
                }
                Event::KeyUp { keycode: Some(kc), .. } => {
                    update_model!(Msg::Input(KeyInput::KeyUp(kc)));
                }
                _ => {}
            }
        }

        let new_now = Instant::now();
        accumulated_time += new_now.duration_since(now).as_secs_f32();
        now = new_now;

        while accumulated_time >= dt {
            accumulated_time -= dt;
            let keyboard_state = event_pump.keyboard_state();
            let input = CurrentInput {
                left: keyboard_state.is_scancode_pressed(Scancode::Left),
                right: keyboard_state.is_scancode_pressed(Scancode::Right),
                up: keyboard_state.is_scancode_pressed(Scancode::Up),
                down: keyboard_state.is_scancode_pressed(Scancode::Down),
            };

            update_model!(Msg::Tick(dt, input));
        }

        view::view(&model, &mut canvas, &assets);

        canvas.present();
    }
}
