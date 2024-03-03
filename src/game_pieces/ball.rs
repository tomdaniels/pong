use crate::constants::*;
use crate::game_pieces::paddle::Paddle;

use rand::Rng;
use raylib::{color::Color, drawing::RaylibDrawHandle, prelude::*};

pub struct Ball {
    pub x:      f32,
    pub y:      f32,
    pub dx:     f32,
    pub dy:     f32,
    pub radius: f32,
    pub speed:  f32,
    pub color:  Color,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Ball {
        let mut rng = rand::thread_rng();

        // delta x/y to track and initialise ball velocity
        let mut dx = 425.0;
        let dy = rng.gen_range::<f32, _>(-250.0..250.0);

        // 50/50 chance of serving left/right
        let exponent = rng.gen_range(0.0..1.0);
        if exponent >= 0.5 { dx = -425.0; }

        Ball {
            x,
            y,
            dx,
            dy,
            radius: 7.0,
            speed:  425.0,
            color:  Color::WHITE
        }
    }

    pub fn render(&self, rl: &mut RaylibDrawHandle<'_>) {
        rl.draw_circle(
            self.x as i32,
            self.y as i32,
            self.radius,
            self.color
        );
    }

    pub fn tick(&mut self, dt: f32) {
        self.x += self.dx * dt;
        self.y += self.dy * dt;

    }

    pub fn wall_hit(&mut self) -> bool {
        // upper boundary detection
        if self.y <= 0.0 {
            self.y = 0.0 + self.radius;
            self.dy = -self.dy;
            return true;
        }

        // lower boundary detection
        if self.y >= WINDOW_HEIGHT - self.radius {
            self.y = WINDOW_HEIGHT - self.radius;
            self.dy = -self.dy;
            return true;
        }

        return false;
    }

    // Axis-Aligned Bounding Box (AABB) collision detection
    pub fn collides(&mut self, paddle: &Paddle) -> bool {
        if self.x > paddle.x + paddle.width || paddle.x > self.x + self.radius {
            return false;
        }

        if self.y > paddle.y + paddle.height || paddle.y > self.y + self.radius {
            return false
        }

        self.continue_trajectory();
        return true;
    }

    fn continue_trajectory(&mut self) {
        let mut rng = rand::thread_rng();

        if self.dy < 0.0 {
            self.dy = -rng.gen_range(10.0..250.0);
        } else {
            self.dy = rng.gen_range(10.0..250.0);
        }
    }

    pub fn reset(&mut self, dx: Option<f32>) {
        self.x = WINDOW_WIDTH_HALF;
        self.y = WINDOW_HEIGHT_HALF;
        self.dx = dx.unwrap_or(self.speed);
    }
}
