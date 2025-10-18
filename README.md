# Silero VAD Rust Implementation (vad-silero-rs)

[![Crates.io](https://img.shields.io/crates/v/vad-silero-rs)](https://crates.io/crates/vad-silero-rs)
[![Docs.rs](https://docs.rs/vad-silero-rs/badge.svg)](https://docs.rs/vad-silero-rs)
[![License](https://img.shields.io/crates/l/vad-silero-rs)](LICENSE)

A high-performance Rust implementation of the [Silero Voice Activity Detection (VAD)](https://github.com/snakers4/silero-vad) model, optimized for real-time audio processing and integration into Rust-based applications. This project aims to provide an efficient and reliable solution for detecting speech segments in audio streams.

## Table of Contents

- [Silero VAD Rust Implementation (vad-silero-rs)](#silero-vad-rust-implementation-vad-silero-rs)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Basic Example](#basic-example)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)

## Features

- **High Performance**: Leverages Rust's performance capabilities for efficient VAD processing.
- **Silero VAD Model Integration**: Directly uses pre-trained Silero VAD models.
- **Real-time Processing**: Designed for low-latency voice activity detection in live audio streams.
- **Easy to Integrate**: Simple API for embedding VAD functionality into your Rust projects.
- **Minimal External Dependencies (Runtime)**: Minimal runtime dependencies for streamlined deployment.
- **Cross-Platform**: Compatible with various operating systems supported by Rust.

## Installation

Add `vad-silero-rs` to your `Cargo.toml` file:

```toml
[dependencies]
vad-silero-rs = "0.1.0" # Use the latest version
```

Or, if you prefer to clone the repository and build from source:

```bash
git clone https://github.com/EliFuzz/vad-silero-rs.git
cd vad-silero-rs
cargo build --release
```

## Usage

The library provides a straightforward interface to initialize the VAD model and process audio chunks.

### Basic Example

Here's a quick example demonstrating how to use `vad-silero-rs` to detect voice activity in an audio buffer:

```rust
use vad_silero_rs::{VadNode, SampleRate, VADProbabilities};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the VAD model with a specific sample rate.
    // Ensure your audio input matches this sample rate.
    let mut vad = VadNode::new(SampleRate::Rate16k)?;

    // Example audio chunk (replace with your actual audio data)
    // This should be a flattened array of f32 samples.
    // For a real application, you would load audio from a file or microphone.
    let audio_chunk: Vec<f32> = vec![0.0; 1024]; // 1024 samples of silence

    // Process the audio chunk
    let probabilities: VADProbabilities = vad.process_audio_chunk(&audio_chunk)?;

    println!("Voice probability: {}", probabilities.speech_probability);
    println!("Silence probability: {}", probabilities.silence_probability);

    if probabilities.is_speech() {
        println!("Speech detected!");
    } else {
        println!("Silence detected.");
    }

    Ok(())
}
```

## License

This project is licensed under the [Apache 2.0 License](LICENSE).

## Acknowledgements

- The original [Silero VAD project](https://github.com/snakers4/silero-vad) for providing the excellent VAD model.
- The Rust community for its robust ecosystem and tools.
