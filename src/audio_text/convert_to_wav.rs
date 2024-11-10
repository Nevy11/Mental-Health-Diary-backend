use hound;

pub fn convert_to_wav(
    input_data: Vec<u8>, // Input audio data in a raw format (e.g., PCM, MP3, etc.)
    output_path: &str,   // Path to save the output WAV file
) -> Result<(), Box<dyn std::error::Error>> {
    // Define the WAV file specifications: 16kHz, 16-bit PCM, mono channel
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,                      // Sample rate of 16kHz
        bits_per_sample: 16,                     // 16-bit audio
        sample_format: hound::SampleFormat::Int, // Integer format (PCM)
    };

    // Create the WAV writer to output to the specified file path
    let mut wav_writer = hound::WavWriter::create(output_path, spec)?;

    // Convert the raw input data (Vec<u8>) into a Vec<i16> for PCM format
    let samples: Vec<i16> = input_data
        .chunks(2) // Each 16-bit sample is 2 bytes
        .map(|chunk| i16::from_ne_bytes([chunk[0], chunk[1]])) // Convert bytes to i16
        .collect();

    // Write each sample to the WAV file
    for sample in samples {
        wav_writer.write_sample(sample)?;
    }

    // Finalize the WAV file
    wav_writer.finalize()?;
    Ok(())
}
