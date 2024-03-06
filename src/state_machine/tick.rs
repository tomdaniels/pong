use crate::constants::*;
use crate::{StateMachine, Phase};

impl<'a> StateMachine<'a> {
    pub fn tick(&mut self, dt: f32) {
        match self.phase {

            Phase::PLAY => {
                self.ball.tick(dt);

                if self.ball.wall_hit() {
                    self.play_sound(&self.audio, "wall_hit");
                }

                if self.ball.collides(&self.p1) {
                    self.play_sound(&self.audio, "ping");
                    self.ball.bounce(self.p1.x + self.p1.width);
                }
                if self.ball.collides(&self.p2) {
                    self.play_sound(&self.audio, "pong");
                    self.ball.bounce(self.p2.x - self.ball.radius);
                }

                let player1_scored = self.ball.x > WINDOW_WIDTH;
                let player2_scored = self.ball.x < 0.0;
                if player1_scored || player2_scored {
                    self.play_sound(&self.audio, "point_scored");
                    self.ball.reset(if player1_scored { Some(-self.ball.dx) } else { None });

                    if player1_scored {
                        self.p1.scored();
                        self.p1.serves = false;
                        self.p2.serves = true;
                    } else {
                        self.p2.scored();
                        self.p2.serves = false;
                        self.p1.serves = true;
                    }

                    self.phase = Phase::SERVE;
                }

                if self.p1.score == SCORE_TARGET || self.p2.score == SCORE_TARGET {
                    self.phase = Phase::DONE;
                }
            }
            _ => {}
        }
    }
}
