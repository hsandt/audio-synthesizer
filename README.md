# Audio synthesizer

A draft of application that generates sine waves and chords to train hearing and playing music/singing.

It is written in Rust and uses [rodio](https://crates.io/crates/rodio) for audio synthesis, [iced](https://crates.io/crates/iced) for the GUI.

## Build

Currently, building requires nightly feature `split_inclusive` and therefore the nightly toolchain.
