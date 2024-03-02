use crate::constants::*;

use raylib::{color::Color, drawing::RaylibDrawHandle, prelude::*};

pub enum Direction { UP, DOWN }

pub struct Paddle {
    pub x:      f32,
    pub y:      f32,
    pub width:  f32,
    pub height: f32,
    pub speed:  f32,
    pub color:  Color,
    pub score:  i32,
    pub serves: bool,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Paddle {
        Paddle {
            x,
            y,
            width:  10.0, 
            height: 80.0,
            speed:  0.15,
            color:  Color::WHITE,
            score:  0,
            serves: false,
        }
    }
    pub fn render(&self, rl: &mut RaylibDrawHandle<'_>) {
        rl.draw_rectangle(
            self.x as i32,
            self.y as i32,
            self.width as i32,
            self.height as i32,
            self.color
        );
    }

    pub fn move_paddle(&mut self, dir: Direction) {
        match dir {
            Direction::DOWN => {
                if self.y < WINDOW_HEIGHT - 80.0 {
                    self.y += self.speed; 
                } 
            },
            Direction::UP => { 
                if self.y > 0.0 {
                    self.y -= self.speed; 
                }
            }
        }
    }

    pub fn scored(&mut self) {
        self.score += 1;
    }
}
