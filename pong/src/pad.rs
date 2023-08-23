use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use crate::PAD_HEIGHT;
use crate::PAD_WIDTH;

pub enum Direction {
    Stop,
    Left,
    Right
}

pub struct Pad {
    x: i32,
    y: i32,

    vel_x: i32
}

impl Pad {
    pub fn new() -> Self {
        Pad {
            x: (SCREEN_WIDTH / 2 - PAD_WIDTH / 2) as i32,
            y: (SCREEN_HEIGHT - PAD_HEIGHT) as i32,
            vel_x: 0
        }
    }

    pub fn set_move(&mut self, dir: Direction) {
        self.vel_x = match dir {
            Direction::Stop => 0,
            Direction::Left => -5,
            Direction::Right => 5
        };
    }

    pub fn update(&mut self) {
        self.x += self.vel_x;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(Rect::new(self.x, self.y, PAD_WIDTH, PAD_HEIGHT)).unwrap();
    }

    pub fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
