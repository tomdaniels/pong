use crate::constants::*;

use raylib::text::measure_text;

fn centered_offset(str: &str, font_size: i32) -> i32 {
    WINDOW_WIDTH_HALF as i32 - measure_text(str, font_size) / 2
}

pub struct Title {
    pub x: i32,
    pub y: i32,
    pub content: String,
    pub font_size: i32,
}

impl Title {
    pub fn new(content: &str, font_size: i32) -> Title {
        Title {
            y: 20,
            x: centered_offset(content, font_size),
            content: String::from(content),
            font_size
        }
    }

    pub fn set_content(&mut self, title: &str) {
        self.content = String::from(title);
        self.x = centered_offset(&self.content, self.font_size);
    }
}
