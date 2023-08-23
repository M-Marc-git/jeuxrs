extern crate sdl2;

use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::pixels::Color;

mod ball;
mod pad;
use ball::Ball;
use pad::Pad;

const SCREEN_WIDTH: u32  = 1024;
const SCREEN_HEIGHT: u32 = 768;
const BALL_SIZE: u32     = 10;
const PAD_WIDTH: u32     = 100;
const PAD_HEIGHT: u32    = 10;

fn main() {
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window("Pong", 1024, 768)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();
    let mut run = true;


    let mut ball = Ball::new();
    let mut pad = Pad::new();
    while run {

        // Remplis la fenetre en blanc
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Affichage des éléments
        ball.draw(&mut canvas);
        pad.draw(&mut canvas);

        canvas.present();

        run = ball.update(&pad);
        pad.update();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => run = false,
                Event::KeyDown { keycode: Some(Keycode::Left),  .. } => pad.set_move(pad::Direction::Left),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => pad.set_move(pad::Direction::Right),
                Event::KeyUp { keycode: Some(Keycode::Left),    .. } => pad.set_move(pad::Direction::Stop),
                Event::KeyUp { keycode: Some(Keycode::Right),   .. } => pad.set_move(pad::Direction::Stop),
                _ => {}
            }
        }

        // Maximum 50 mise à jour par seconde
        std::thread::sleep(Duration::new(0, 1000000000 / 50));
    }
}
