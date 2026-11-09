use crate::audio::lecture_audio;
use crate::default_audio::BELL_SOUND;

mod default_audio;
mod time;
mod ui;
mod audio;



fn main() {
    println!("Hello, world!");

    let audio  = lecture_audio::AudioPlayer::new(BELL_SOUND);
    audio.play();
    

}
