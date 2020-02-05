use std::time::Instant;
use std::fs::File;

use sdl2::event::Event;
use sdl2::pixels::Color;

mod msg;
mod model;
mod input;
mod view;
mod inventory;

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
    let mut model = Model::init();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Nigel Wormclams Magic Adventure", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 50, 80));
    canvas.clear();
    canvas.present();

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

        view::view(&model, &mut canvas);

        canvas.present();
    }
}
