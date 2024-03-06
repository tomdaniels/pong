use raylib::prelude::*;
use crate::{game_pieces::Direction, Phase, StateMachine};

impl<'a> StateMachine<'a> {
    pub fn handle_input(&mut self, rl: &RaylibHandle) {
        match self.phase {
            Phase::IDLE => {
                if rl.is_key_down(KeyboardKey::KEY_ENTER) || rl.is_key_down(KeyboardKey::KEY_SPACE) {
                    self.play_sound(&self.audio, "serve");
                    self.phase = Phase::PLAY;
                }
            }

            Phase::PLAY => {
                if rl.is_key_down(KeyboardKey::KEY_W) {
                    self.p1.move_paddle(Direction::UP);
                }
                if rl.is_key_down(KeyboardKey::KEY_S) {
                    self.p1.move_paddle(Direction::DOWN);
                }

                if rl.is_key_down(KeyboardKey::KEY_UP) {
                    self.p2.move_paddle(Direction::UP);
                }
                if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                    self.p2.move_paddle(Direction::DOWN);
                }
            }

            Phase::SERVE => {
                if self.p1.serves && (rl.is_key_down(KeyboardKey::KEY_SPACE)) {
                    self.play_sound(&self.audio, "serve");
                    self.ball.reset(None);
                    self.phase = Phase::PLAY;
                } else if self.p2.serves && (rl.is_key_down(KeyboardKey::KEY_ENTER)) {
                    self.play_sound(&self.audio, "serve");
                    self.ball.reset(Some(-self.ball.speed));
                    self.phase = Phase::PLAY;
                }
            }

            Phase::DONE => {
                if rl.is_key_down(KeyboardKey::KEY_ENTER) || rl.is_key_down(KeyboardKey::KEY_SPACE) {
                    self.p1.score = 0;
                    self.p2.score = 0;
                    self.phase = Phase::IDLE;
                }
            }
        }
    }
}
