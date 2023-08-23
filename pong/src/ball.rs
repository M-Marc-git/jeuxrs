use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::BALL_SIZE;
use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;
use crate::PAD_WIDTH;
use crate::PAD_HEIGHT;
use crate::pad::Pad;

pub struct Ball {
    x: i32,
    y: i32,

    vel_x: i32,
    vel_y: i32,

    score: i32
}

impl Ball {
    pub fn new() -> Self {
        Self {
            x: (SCREEN_WIDTH / 2 - BALL_SIZE / 2) as i32,
            y: (SCREEN_HEIGHT / 2 - BALL_SIZE / 2) as i32,
            vel_x: if rand::random() { 3 } else { -3 },
            vel_y: if rand::random() { 3 } else { -3 },
            score: 0
        }
    }

    pub fn update(&mut self, pad: &Pad) -> bool {
        // Déplacement de la balle
        self.x += self.vel_x;
        self.y += self.vel_y;

        // Colision avec les bords verticaux de l'écran
        if (self.x + BALL_SIZE as i32) >= SCREEN_WIDTH as i32 || self.x <= 0 {
            self.vel_x = -self.vel_x;
        }

        // Colision avec le haut de l'écran
        if self.y <= 0 {
            self.vel_y = -self.vel_y;
        }

        // Colision avec la raquette
        let pad_pos = pad.get_pos();
        if self.x >= pad_pos.0 && self.y >= pad_pos.1 && 
                self.x <= (pad_pos.0 + PAD_WIDTH as i32) && 
                self.y <= (pad_pos.1 + PAD_HEIGHT as i32) {
            self.vel_y = -self.vel_y;
            self.score += 1;
            return true;
        }
        
        // Colision avec le bas de l'écran
        if self.y + BALL_SIZE as i32 >= SCREEN_HEIGHT as i32 {
            false
        } else { 
            true
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(Rect::new(self.x, self.y, BALL_SIZE, BALL_SIZE)).unwrap();
    }
}

impl Drop for Ball {
    fn drop(&mut self) {
        println!("Score: {}", self.score);
    }
}
