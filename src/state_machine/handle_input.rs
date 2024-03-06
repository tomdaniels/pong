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
                    serve_ball(self, None);
                } else if self.p2.serves && (rl.is_key_down(KeyboardKey::KEY_ENTER)) {
                    serve_ball(self, Some(-self.ball.speed));
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

fn serve_ball(machine: &mut StateMachine, dx_offset: Option<f32>) {
    machine.play_sound(&machine.audio, "serve");
    machine.ball.reset(dx_offset);
    machine.phase = Phase::PLAY;
}
