use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

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
        let sound = BufReader::new(File::open(format!("src/assets/{}.wav", filename)).unwrap());
        self.sink.append(rodio::Decoder::new(BufReader::new(sound)).unwrap());
    }
}
