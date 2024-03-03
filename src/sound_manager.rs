use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub struct SoundManager {
    sink: Sink
}

impl SoundManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        SoundManager {
            sink
        }
    }

    pub fn play_sound(&self, filename: &str) {
        let file = format!("src/assets/{}.wav", filename);
        let sound = BufReader::new(File::open(file).unwrap());
        let source = Decoder::new(BufReader::new(sound)).unwrap();
        self.sink.append(source);
    }
}
