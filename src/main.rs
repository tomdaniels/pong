mod constants;
use constants::*;

mod game_pieces;
mod state_machine;
use state_machine::*;

use rodio::{OutputStream, Sink};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let audio_stream = Sink::try_new(&stream_handle).unwrap();

    let mut game = StateMachine::new(&audio_stream);

    let (mut rayl, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("Pong")
        .build();

    while !rayl.window_should_close() {
        let mut rl = rayl.begin_drawing(&thread);
        let dt = rl.get_frame_time();

        game.handle_input(&rl);
        game.tick(dt);
        game.render(&mut rl);
    }
}

