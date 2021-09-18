#![feature(split_inclusive)]
mod app_message;
mod app_state;

use app_state::AppState;
use iced::{Sandbox, Settings};

fn main() {
    AppState::run(Settings::default()).expect("App run failed");

    loop {}
}
