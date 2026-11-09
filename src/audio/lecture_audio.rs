use std::io::{BufReader, Cursor, Sink};
use rodio::{Decoder, MixerDeviceSink};
use rodio::DeviceSinkBuilder;

pub struct AudioPlayer{
    audio_to_play : &'static [u8],
    sink : MixerDeviceSink
}


impl AudioPlayer {
    pub fn new(audio_bytes : &'static[u8]) -> Self{
        let mixer = DeviceSinkBuilder::open_default_sink().expect("ee");
        Self{audio_to_play : audio_bytes,sink : mixer }
    }

    pub fn play(&self) {
        let cursor = Cursor::new(self.audio_to_play);
        let source = BufReader::new(cursor);
        let player = rodio::play(&self.sink.mixer(), source).unwrap();
        player.sleep_until_end();
    }
}