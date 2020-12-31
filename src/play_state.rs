use crate::play_message::PlayMessage;
use iced::{button, Element, Sandbox, Text};

#[derive(Default)]
pub struct PlayState {
    // Are we playing the sine wave?
    is_playing: bool,

    // Local state of the play button
    play_button: button::State,
}

impl Sandbox for PlayState {
    type Message = ();

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Title")
    }
    fn update(&mut self, _message: Self::Message) {}

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello").into()
    }
}
