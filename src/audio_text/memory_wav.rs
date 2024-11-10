use super::create_audio_text::whisper_transcribe;

pub fn convert_to_wav_in_memory(
    input_data: Vec<u8>,
    output: &mut Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = std::io::Cursor::new(output);
    let mut writer = hound::WavWriter::new(&mut cursor, spec)?;

    for chunk in input_data.chunks(2) {
        if chunk.len() < 2 {
            continue; // Skip any incomplete chunk to avoid index issues
        }
        let sample = i16::from_ne_bytes([chunk[0], chunk[1]]);
        writer.write_sample(sample)?;
    }

    writer.finalize()?;
    Ok(())
}

pub fn transcribe_audio_from_memory(wav_data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let cursor = std::io::Cursor::new(wav_data);
    let mut reader = hound::WavReader::new(cursor)?;
    let audio_data: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    println!("Audio data length: {}", audio_data.len());

    whisper_transcribe(&audio_data)
}
