use crate::play_message::PlayMessage;
use iced::{button, Button, Column, Container, Element, Row, Sandbox, Text};

pub struct PlayState {
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

impl Sandbox for PlayState {
    type Message = PlayMessage;

    fn new() -> Self {
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let mut sine_wave_states = Vec::new();

        // I can't find a way to stop() a sink and make it work again
        // even when calling append() and play(), so for now I just have a list
        // or sinks ready to play at different frequencies and just pause/play
        // them as needed. They must all start paused.

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
        // let frequencies = [440, 554, 659];
        let frequencies = [440.00, 554.37, 659.25];

        // fill sine wave state with sink and default state for each frequency
        for i in 0..frequencies.len() {
            let sink = rodio::Sink::try_new(&stream_handle).unwrap();
            let source = rodio::source::SineWave::new(frequencies[i]);
            sink.append(source);
            sink.pause();

            sine_wave_states.push(SineWaveState {
                sink,
                is_playing: false,
                play_button: button::State::default(),
            });
        }

        Self {
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
            PlayMessage::TogglePlayback(freq_index) => {
                if self.sine_wave_states[freq_index].is_playing {
                    self.pause_and_normalize(freq_index)
                } else {
                    self.play_and_normalize(freq_index)
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut controls = Row::new();

        // explode the state in individual mutable slices containing 1 element each,
        // just so we can use them in the loop and push them as mutable without causing
        // borrowing conflicts with competing usages in the loop
        let sine_wave_state_solo_slice_iter = self.sine_wave_states.chunks_mut(1);

        for (i, sine_wave_state_solo_slice) in sine_wave_state_solo_slice_iter.enumerate() {
            assert_eq!(
                sine_wave_state_solo_slice.len(),
                1,
                "chunks_mut(1) should generate slices of length 1"
            );
            let sine_wave_state = &mut sine_wave_state_solo_slice[0];
            controls = controls.push(
                Button::new(
                    &mut sine_wave_state.play_button,
                    Text::new(if sine_wave_state.is_playing {
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
