use crate::play_message::PlayMessage;
use iced::{button, Button, Column, Container, Element, Sandbox, Text};

#[derive(Default)]
pub struct PlayState {
    // OutputStream to play sound
    stream: Option<rodio::OutputStream>,

    // Sink where sine waves are played
    sink: Option<rodio::Sink>,

    // Are we playing the sine wave?
    is_playing: bool,

    // Local state of the play button
    play_button: button::State,
}

impl Sandbox for PlayState {
    type Message = PlayMessage;

    fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();

        // I can't find a way to stop() a sink and make it work again
        // even when calling append() and play(), so for now I just have a list
        // or sinks ready to play at different frequencies and just pause/play
        // them as needed. They must all start paused.

        // https://pages.mtu.edu/~suits/notefreqs.html
        let source = rodio::source::SineWave::new(440); // A4
        sink.append(source);
        sink.pause();

        Self {
            stream: Some(stream),
            sink: Some(sink),
            ..Self::default()
        }
    }

    fn title(&self) -> String {
        String::from("Title")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            PlayMessage::TogglePlayback => {
                self.is_playing ^= true;
                println!("pressed: {}", self.is_playing);
                if self.is_playing {
                    self.play_sine_wave()
                } else {
                    self.pause_sink()
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        Button::new(
            &mut self.play_button,
            Text::new(if self.is_playing { "Pause" } else { "Play" }),
        )
        .on_press(PlayMessage::TogglePlayback)
        .into()
    }
}

impl PlayState {
    fn play_sine_wave(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    fn pause_sink(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }
}
