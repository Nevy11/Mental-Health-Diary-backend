use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub fn whisper_transcribe(audio_data: &[f32]) -> Result<String, Box<dyn std::error::Error>> {
    if audio_data.is_empty() {
        return Err("Audio data is empty; cannot proceed with transcription".into());
    }

    let path_to_model: String = String::from("whisper.cpp/models/ggml-base.en.bin");

    let ctx = WhisperContext::new_with_params(&path_to_model, WhisperContextParameters::default())
        .expect("failed to load model");

    let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    let mut state = ctx.create_state().expect("failed to create state");
    state.full(params, audio_data)?;

    let num_segments = state.full_n_segments()?;
    let mut transcription = String::new();
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)?;
        transcription.push_str(&segment);
    }
    Ok(transcription)
}
