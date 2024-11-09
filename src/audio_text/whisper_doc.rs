use hound;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters}; // Hound library for handling WAV files

/// Whisper medium english model takes in the path of an audio, reads the audio file
/// from where it is stored. The model then loads the whisper model from where it is
/// stored in the folder as a bin file.
pub fn whisper_transcribe_medium() -> Result<String, Box<dyn std::error::Error>> {
    let path_to_model: String = String::from("whisper.cpp/models/ggml-medium.en.bin");

    // Load the Whisper model
    let ctx = WhisperContext::new_with_params(&path_to_model, WhisperContextParameters::default())
        .expect("failed to load model");

    // Set the transcription parameters
    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // Load audio data from a WAV file and convert to f32
    let audio_path = "whisper.cpp/samples/jfk.wav";
    let mut reader = hound::WavReader::open(audio_path)?;

    // Convert audio samples from 16-bit PCM to normalized f32 values
    let audio_data: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    // Run the model
    let mut state = ctx.create_state().expect("failed to create state");
    state.full(params, &audio_data[..])?;

    // Collect and return the transcribed text
    let num_segments = state.full_n_segments()?;
    let mut transcription = String::new();
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)?;
        transcription.push_str(&segment);
    }
    Ok(transcription)
}
