use hound;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters}; // Hound library for handling WAV files

pub fn whisper_doc2() {
    let path_to_model = String::from("whisper.cpp/models/ggml-base.en.bin");

    // Load a context and model
    let ctx = WhisperContext::new_with_params(&path_to_model, WhisperContextParameters::default())
        .expect("failed to load model");

    // Create a params object
    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    // Load audio data from a WAV file and convert it to f32
    let audio_path = "whisper.cpp/samples/jfk.wav";
    let mut reader = hound::WavReader::open(audio_path).expect("Failed to open audio file");

    // Extract audio samples, assuming 16-bit PCM mono audio
    let audio_data: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32) // Normalize samples to range [-1.0, 1.0]
        .collect();

    // Run the model
    let mut state = ctx.create_state().expect("failed to create state");
    state
        .full(params, &audio_data[..])
        .expect("failed to run model");

    // Fetch and print the results
    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");
    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to get segment");
        let start_timestamp = state
            .full_get_segment_t0(i)
            .expect("failed to get segment start timestamp");
        let end_timestamp = state
            .full_get_segment_t1(i)
            .expect("failed to get segment end timestamp");
        println!("[{} - {}]: {}", start_timestamp, end_timestamp, segment);
    }
}
