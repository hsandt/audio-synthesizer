#![feature(split_inclusive)]
mod play_message;
mod play_state;

use iced::{Sandbox, Settings};
use play_state::PlayState;

fn main() {
    PlayState::run(Settings::default()).expect("App run failed");

    loop {}
}
