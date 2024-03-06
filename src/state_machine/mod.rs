use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, Sink};

use crate::constants::*;
use crate::game_pieces::{Paddle, Ball};

pub enum Phase { IDLE, PLAY, SERVE, DONE }

mod handle_input;
mod render;
mod tick;

pub struct StateMachine<'a> {
    p1: Paddle,
    p2: Paddle,
    ball: Ball,
    phase: Phase,
    audio: &'a Sink,
}

impl<'a> StateMachine<'a> {
    pub fn new(audio: &'a Sink) -> StateMachine<'a> {
        let p1 = Paddle::new(3.0, 3.0);
        let p2 = Paddle::new(WINDOW_WIDTH - p1.width - 3.0, WINDOW_HEIGHT - p1.height - 3.0);
        let ball = Ball::new(WINDOW_WIDTH_HALF, WINDOW_HEIGHT_HALF);

        Self { p1, p2, ball, phase: Phase::IDLE, audio }
    }

    fn play_sound(&mut self, audio: &Sink, filename: &str) {
        let file = format!("assets/{}.wav", filename);
        let sound = BufReader::new(File::open(file).unwrap());
        let source = Decoder::new(sound).unwrap();
        audio.append(source);
    }
}
