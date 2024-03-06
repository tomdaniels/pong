use crate::constants::*;

use raylib::{drawing::RaylibDrawHandle, prelude::*};

fn centered_offset(str: &str, font_size: i32) -> i32 {
    WINDOW_WIDTH_HALF as i32 - measure_text(str, font_size) / 2
}

pub struct Title {
    pub x:         i32,
    pub y:         i32,
    pub content:   String,
    pub font_size: i32,
}

impl Title {
    pub fn new(content: &str, font_size: i32 ) -> Title {
        Title {
            y: 20,
            x: centered_offset(content, font_size),
            content: String::from(content),
            font_size,
        }
    }

    pub fn draw(&self, rl: &mut RaylibDrawHandle<'_>, color: Color, vertical_placement: Option<i32>) {
        rl.draw_text(&self.content, self.x, vertical_placement.unwrap_or(self.y), self.font_size, color);
    }
}
