use crate::play_message::PlayMessage;
use iced::{button, Button, Column, Container, Element, Row, Sandbox, Text};
use std::cell::RefCell;

pub struct PlayState {
    // OutputStream to play sound
    stream: rodio::OutputStream,

    // Vector of states of the sine wave generators
    // (array default initialization requires Copy, which Sink doesn't provide)
    sine_wave_states: RefCell<Vec<SineWaveState>>,
}

struct SineWaveState {
    // Sink where sine waves are played
    sink: rodio::Sink,

    // Are we playing the sine wave?
    is_playing: bool,

    // Local state of the play button
    play_button: button::State,
}

impl Sandbox for PlayState {
    type Message = PlayMessage;

    fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let mut sine_wave_states = RefCell::new(Vec::new());

        // I can't find a way to stop() a sink and make it work again
        // even when calling append() and play(), so for now I just have a list
        // or sinks ready to play at different frequencies and just pause/play
        // them as needed. They must all start paused.

        // https://pages.mtu.edu/~suits/notefreqs.html
        // we must round to nearest integer due to limitation of SineWave
        // (see https://github.com/RustAudio/rodio/issues/187)
        // A4       440.00
        // A#4/Bb4  466.16
        // B4       493.88
        // C5       523.25
        let frequencies = [440, 494];

        // fill sine wave state with sink and default state for each frequency
        for i in 0..frequencies.len() {
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let source = rodio::source::SineWave::new(frequencies[i]);
            sink.append(source);
            sink.pause();

            sine_wave_states.borrow_mut()[i] = SineWaveState {
                sink,
                is_playing: false,
                play_button: button::State::default(),
            };
        }

        Self {
            stream,
            sine_wave_states,
        }
    }

    fn title(&self) -> String {
        String::from("Title")
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            PlayMessage::TogglePlayback(freq_index) => {
                self.sine_wave_states.borrow_mut()[freq_index].is_playing ^= true;
                if self.sine_wave_states.borrow()[freq_index].is_playing {
                    self.play_sine_wave(freq_index)
                } else {
                    self.pause_sink(freq_index)
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut controls = Row::new();

        for i in 0..self.sine_wave_states.borrow().len() {
            controls = controls.push(
                Button::new(
                    &mut self.sine_wave_states.borrow_mut()[i].play_button,
                    Text::new(if self.sine_wave_states.borrow()[i].is_playing {
                        "Pause"
                    } else {
                        "Play"
                    }),
                )
                .on_press(PlayMessage::TogglePlayback(i)),
            );
        }

        controls.into()
    }
}

impl PlayState {
    fn play_sine_wave(&self, freq_index: usize) {
        &self.sine_wave_states.borrow()[freq_index].sink.play();
    }

    fn pause_sink(&self, freq_index: usize) {
        &self.sine_wave_states.borrow()[freq_index].sink.pause();
    }
}
