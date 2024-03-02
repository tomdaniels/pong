mod constants;
mod game_pieces;
mod sound_manager;

use constants::*;
use game_pieces::{Paddle, Direction, Ball, Title};
use sound_manager::SoundManager;

use raylib::{color::Color, prelude::*};

enum GameState { IDLE, PLAY, SERVE, DONE }

fn draw_scores(rl: &mut RaylibDrawHandle<'_>, score1: &i32, score2: &i32) {
    rl.draw_text(&score1.to_string(), WINDOW_WIDTH as i32 / 8, 125, FONT_SIZE * 3, Color::WHITE);
    rl.draw_text(&score2.to_string(), WINDOW_WIDTH as i32 - WINDOW_WIDTH as i32 / 8, 125, FONT_SIZE * 3, Color::WHITE);
}

fn main() {
    let sm = SoundManager::new();

    let mut ball = Ball::new(WINDOW_WIDTH_HALF, WINDOW_HEIGHT_HALF);
    let mut p1 = Paddle::new(3.0, 3.0);
    let mut p2 = Paddle::new(WINDOW_WIDTH - p1.width - 3.0, WINDOW_HEIGHT - p1.height - 3.0);

    let mut game_state = GameState::IDLE;

    let (mut rayl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Pong")
        .build();

    while !rayl.window_should_close() {
        let mut rl = rayl.begin_drawing(&thread);
        let dt = rl.get_frame_time();
        rl.clear_background(Color::BLACK);

        let main_title = Title::new("Press [Enter] or [Space] to serve", FONT_SIZE);
        let mut serve_title = Title::new("Player 1 scored", FONT_SIZE * 2);
        let mut serve_subtitle = Title::new("Player 2: Press [Enter] to serve", FONT_SIZE);
        let mut winner_title = Title::new("Player 1 wins", FONT_SIZE * 2);
        let restart_title  = Title::new("Press [Enter] or [Space] to start again", FONT_SIZE);

        match game_state {
            GameState::IDLE => {
                rl.draw_text(&main_title.content, main_title.x, main_title.y, FONT_SIZE, Color::WHITE);
                if rl.is_key_down(KeyboardKey::KEY_ENTER) || rl.is_key_down(KeyboardKey::KEY_SPACE) {
                    game_state = GameState::PLAY; 
                }
            },

            GameState::PLAY => {
                draw_scores(&mut rl, &p1.score, &p2.score);
                ball.tick(dt, &sm);

                if rl.is_key_down(KeyboardKey::KEY_W) {
                    p1.move_paddle(Direction::UP);
                }
                if rl.is_key_down(KeyboardKey::KEY_S) {
                    p1.move_paddle(Direction::DOWN); 
                }

                if rl.is_key_down(KeyboardKey::KEY_UP)   {
                    p2.move_paddle(Direction::UP);   
                }
                if rl.is_key_down(KeyboardKey::KEY_DOWN) {
                    p2.move_paddle(Direction::DOWN); 
                }

                if ball.collides(&p1) {
                    // sm.play_sound("paddle_hit");
                    ball.dx = -ball.dx * 1.03;
                    ball.x = p1.x + p1.width;
                }
                if ball.collides(&p2) {
                    // sm.play_sound("paddle_hit");
                    ball.dx = -ball.dx * 1.03;
                    ball.x = p2.x - ball.radius;
                }

                let player1_scored = ball.x > WINDOW_WIDTH;
                let player2_scored = ball.x < 0.0;
                if player1_scored || player2_scored {
                    // sm.play_sound("point_scored");
                    if player1_scored {
                        p1.scored();
                        p1.serves = false;
                        p2.serves = true;
                    }
                    if player2_scored {
                        p2.scored();
                        p2.serves = false;
                        p1.serves = true;
                    }

                    ball.dx = -ball.dx;
                    game_state = GameState::SERVE;
                }

                if p1.score == SCORE_TARGET || p2.score == SCORE_TARGET {
                    game_state = GameState::DONE;
                }
            },

            GameState::SERVE => {
                // sm.play_sound("wall_hit");
                draw_scores(&mut rl, &p1.score, &p2.score);
                ball.reset();

                if p1.serves {
                    serve_title.set_content("player 2 scored");
                    serve_subtitle.set_content("player 1: press [Space] to serve");

                    if rl.is_key_down(KeyboardKey::KEY_SPACE) {
                        game_state = GameState::PLAY; 
                    }
                } else if p2.serves {
                    serve_title.set_content("player 1 scored");
                    serve_subtitle.set_content("player 2: press [Enter] to serve");

                    if rl.is_key_down(KeyboardKey::KEY_ENTER) {
                        game_state = GameState::PLAY; 
                    }
                }

                let vertical_offset = 125;
                rl.draw_text(
                    &serve_title.content, 
                    serve_title.x, 
                    vertical_offset + serve_title.y,
                    serve_title.font_size,
                    Color::WHITE
                );
                rl.draw_text(&serve_subtitle.content,
                    serve_subtitle.x,
                    vertical_offset + (serve_subtitle.y * 2) + serve_title.font_size,
                    serve_subtitle.font_size, Color::WHITE
                );
            }

            GameState::DONE => {
                draw_scores(&mut rl, &p1.score, &p2.score);
                ball.reset();

                if p2.score == SCORE_TARGET {
                    winner_title.set_content("Player 2 wins");
                }
                rl.draw_text(&winner_title.content, winner_title.x, winner_title.y, winner_title.font_size, Color::GREEN);
                rl.draw_text(
                    &restart_title.content,
                    restart_title.x,
                    restart_title.y + winner_title.font_size + 5,
                    restart_title.font_size,
                    Color::WHITE
                );

                if rl.is_key_down(KeyboardKey::KEY_ENTER) || rl.is_key_down(KeyboardKey::KEY_SPACE) {
                    p1.score = 0;
                    p2.score = 0;
                    game_state = GameState::IDLE; 
                }
            }
        }

        ball.render(&mut rl);
        p1  .render(&mut rl);
        p2  .render(&mut rl);
    }
}