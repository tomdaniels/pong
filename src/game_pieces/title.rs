use crate::constants::*;

use raylib::text::measure_text;

pub struct Title {
    pub x: i32,
    pub y: i32,
    pub content: String,
    pub font_size: i32,
}

impl Title {
    pub fn new(content: &str, font_size: i32) -> Title {
        Title {
            x: WINDOW_WIDTH_HALF as i32 - (measure_text(&content, font_size) / 2),
            y: 20,
            content: String::from(content),
            font_size
        }
    }

    pub fn set_content(&mut self, title: &str) {
        self.content = String::from(title);
        self.x = WINDOW_WIDTH_HALF as i32 - measure_text(&self.content, self.font_size) / 2;
    }
}
