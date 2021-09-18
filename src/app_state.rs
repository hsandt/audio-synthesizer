use crate::app_message::AppMessage;
use iced::{button, Button, Column, Container, Element, Row, Sandbox, Text};

// https://pages.mtu.edu/~suits/notefreqs.html
// we must round to nearest integer due to limitation of SineWave
// (see https://github.com/RustAudio/rodio/issues/187)
// Note     Frequency (Hz)
// A4       440.00
// A#4/Bb4  466.16
// B4       493.88
// C5       523.25
// C#5/Db5  554.37
// D5       587.33
// D#5/Eb5  622.25
// E5       659.25
// const FREQUENCIES: [u32; 3] = [440, 554, 659];
// with local patch to allow f32 value
const FREQUENCIES: [f32; 3] = [440.00, 554.37, 659.25];

#[derive(Debug)]
enum AppScreen {
    Main,
    Sandbox,
}

pub struct AppState {
    /// Current screen
    current_screen: AppScreen,

    /// Button states
    sandbox_button_state: button::State,

    /* Sandbox */
    /// OutputStream to play sound
    stream: rodio::OutputStream,

    /// Vector of states of the sine wave generators
    /// (array default initialization requires Copy, which Sink doesn't provide)
    sine_wave_states: Vec<SineWaveState>,

    /// Cached number of playing sinks. Used to normalize volume
    cached_playing_sink_count: i8,
}

struct SineWaveState {
    /// Sink where sine waves are played
    sink: rodio::Sink,

    /// Are we playing the sine wave?
    is_playing: bool,

    /// Local state of the play button
    play_button: button::State,
}

impl Sandbox for AppState {
    type Message = AppMessage;

    fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let mut sine_wave_states = Vec::new();

        // I can't find a way to stop() a sink and make it work again
        // even when calling append() and play(), so for now I just have a list
        // or sinks ready to play at different frequencies and just pause/play
        // them as needed. They must all start paused.

        // fill sine wave state with sink and default state for each frequency
        for i in 0..FREQUENCIES.len() {
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let source = rodio::source::SineWave::new(FREQUENCIES[i]);
            sink.append(source);
            sink.pause();

            sine_wave_states.push(SineWaveState {
                sink,
                is_playing: false,
                play_button: button::State::new(),
            });
        }

        Self {
            current_screen: AppScreen::Main,
            sandbox_button_state: button::State::new(),
            stream,
            sine_wave_states,
            cached_playing_sink_count: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Title")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            AppMessage::EnterSandboxMode => {
                self.current_screen = AppScreen::Sandbox;
            }
            AppMessage::ExitSandboxMode => {
                self.current_screen = AppScreen::Main;
            }
            AppMessage::TogglePlayback(freq_index) => {
                if self.sine_wave_states[freq_index].is_playing {
                    self.pause_and_normalize(freq_index);
                } else {
                    self.play_and_normalize(freq_index);
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        match self.current_screen {
            AppScreen::Main => {
                let mut controls = Row::new();
                controls = controls.push(
                    Button::new(&mut self.sandbox_button_state, Text::new("Sandox mode"))
                        .on_press(AppMessage::EnterSandboxMode),
                );
                controls.into()
            }
            AppScreen::Sandbox => {
                let mut controls = Column::new();

                let mut sandbox_controls = Row::new();

                // in order to share mutable references to each element of the vector
                // we get the full slice and cut it 1 element at a time from the start,
                // passing the reference to that element's play_button to make a Button
                let mut rest = &mut self.sine_wave_states[..];
                let mut i = 0;

                while let Some((sine_wave_state, next_rest)) = rest.split_first_mut() {
                    rest = next_rest;
                    sandbox_controls = sandbox_controls.push(
                        Button::new(
                            &mut sine_wave_state.play_button,
                            Text::new(if sine_wave_state.is_playing {
                                format!("Pause {} Hz", FREQUENCIES[i])
                            } else {
                                format!("Play {} Hz", FREQUENCIES[i])
                            }),
                        )
                        .on_press(AppMessage::TogglePlayback(i)),
                    );
                    i += 1;
                }

                // push moves content, so always call it after defining the full sub-widget
                controls = controls.push(sandbox_controls);

                // Back button
                controls = controls.push(
                    Button::new(&mut self.sandbox_button_state, Text::new("Back"))
                        .on_press(AppMessage::ExitSandboxMode),
                );

                controls.into()
            }
        }
    }
}

impl AppState {
    fn play_and_normalize(&mut self, freq_index: usize) {
        self.sine_wave_states[freq_index].sink.play();
        self.sine_wave_states[freq_index].is_playing = true;
        self.cached_playing_sink_count += 1;
        self.normalize_volume();
    }

    fn pause_and_normalize(&mut self, freq_index: usize) {
        self.sine_wave_states[freq_index].sink.pause();
        self.sine_wave_states[freq_index].is_playing = false;
        self.cached_playing_sink_count -= 1;
        self.normalize_volume();
    }

    /// Normalize volume based on number of playing sink
    /// This is important to avoid saturation which breaks chords completely
    fn normalize_volume(&self) {
        if self.cached_playing_sink_count > 0 {
            for sine_wave_state in &self.sine_wave_states {
                if sine_wave_state.is_playing {
                    let normalized_volume = 1. / self.cached_playing_sink_count as f32;
                    sine_wave_state.sink.set_volume(normalized_volume);
                }
            }
        }
    }
}
