mod play_message;
mod play_state;

use iced::{Sandbox, Settings};
use play_state::PlayState;

fn main() {
    // let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    // let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    // // https://pages.mtu.edu/~suits/notefreqs.html
    // let source = rodio::source::SineWave::new(440); // A4

    // sink.append(source);

    PlayState::run(Settings::default());

    loop {}
}
