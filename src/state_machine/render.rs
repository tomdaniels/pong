mod render {
    use raylib::prelude::*;
    use crate::constants::*;
    use crate::{StateMachine, Phase, game_pieces::Title};

    impl<'a> StateMachine<'a> {
        pub fn render(&mut self, rl: &mut RaylibDrawHandle) {
            rl.clear_background(Color::BLACK);

            match self.phase {
                Phase::IDLE => {
                    Title::new("Press [Enter] or [Space] to serve", FONT_SIZE)
                        .draw(rl, Color::WHITE, None);
                }

                Phase::PLAY => {
                    draw_scores(rl, self.p1.score, self.p2.score);
                }

                Phase::SERVE => {
                    draw_scores(rl, self.p1.score, self.p2.score);
                    let serve_title = get_serve_title(&self.p1.serves);
                    let serve_subtitle = get_serve_subtitle(&self.p1.serves);
                    Title::new(serve_title, FONT_SIZE_LARGE).draw(rl, Color::WHITE, Some(VERTICAL_TEXT_OFFSET));
                    Title::new(serve_subtitle, FONT_SIZE).draw(rl, Color::WHITE, Some(VERTICAL_TEXT_OFFSET + FONT_SIZE_LARGE));
                }

                Phase::DONE => {
                    draw_scores(rl, self.p1.score, self.p2.score);
                    let winner = if self.p1.score == SCORE_TARGET { "Player 1 won!" } else { "Player 2 won!" };
                    Title::new(winner, FONT_SIZE_LARGE).draw(rl, Color::GREEN, Some(VERTICAL_TEXT_OFFSET));
                    Title::new("Press [Enter] or [Space] to start again", FONT_SIZE).draw(rl, Color::WHITE, Some(VERTICAL_TEXT_OFFSET + FONT_SIZE_LARGE));
                }
            }

            self.ball.render(rl);
            self.p1.render(rl);
            self.p2.render(rl);
        }

    }

    fn get_serve_title(is_p1_serving: &bool) -> &str {
        if *is_p1_serving { "Player 2 Scored!" } else { "Player 1 Scored!" }
    }

    fn get_serve_subtitle(is_p1_serving: &bool) -> &str {
        if *is_p1_serving { "Player 1: Press [Space] to serve" } else { "Player 2: Press [Enter] to serve" }
    }

    fn draw_scores(rl: &mut RaylibDrawHandle, score1: i32, score2: i32) {
        rl.draw_text(
            &score1.to_string(),
            WINDOW_WIDTH as i32 / 8,
            VERTICAL_TEXT_OFFSET,
            FONT_SIZE * 3, Color::WHITE
        );
        rl.draw_text(
            &score2.to_string(),
            WINDOW_WIDTH as i32 - WINDOW_WIDTH as i32 / 8,
            VERTICAL_TEXT_OFFSET,
            FONT_SIZE * 3, Color::WHITE
        );
    }
}
