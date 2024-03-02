use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};

/**
* TODO: 
*               how to fire single play of sounds in rust..
*               expose the API of sound_manager.play_sound("filename");
*
*               rodio seems promising but files play in a separate thread
*               and once the sounds start playing, the main thread freezes.
* */

pub struct SoundManager {
    sounds: HashMap<String, Decoder<BufReader<File>>>,
    stream_handle: OutputStreamHandle,
}

impl SoundManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut sound_manager = Self {
            sounds: HashMap::new(),
            stream_handle,
        };

        // Load sounds during initialization
        sound_manager.load_sound("paddle_hit");
        sound_manager.load_sound("wall_hit");
        sound_manager.load_sound("point_scored");

        sound_manager
    }

    fn load_sound(&mut self, filename: &str) {
        let file = File::open(format!("src/assets/{}.wav", filename)).unwrap();
        let decoder = Decoder::new(BufReader::new(file)).unwrap();
        self.sounds.insert(filename.to_string(), decoder);
    }

    pub fn play_sound(&self, filename: &str) {
        // if let Some(source) = self.sounds.get(filename) {
        //     let cloned = source.clone();
        //     let _ = self.stream_handle.play_raw(source);
        // } else {
        //     println!("Sound {} not found", filename);
        // }
    }
}
