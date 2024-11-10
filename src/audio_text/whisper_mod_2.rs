use actix_multipart::Multipart;
use actix_web::web;
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::str;

/// This function saves the audio in the target folder because rust actix server
/// does not restart the server{doesn't check the file}.
/// Takes in a stream of data as a Multipart, then loops over every stream of data
/// until it is None. Creates a new file `target/uploads/audio.wav` and if there is,
/// it updates the data.
/// The function then calls convert wav function.
pub async fn save_audio(mut payload: Multipart) -> std::io::Result<String> {
    let mut file_path = String::new();

    // Process each field in the multipart payload
    while let Ok(Some(mut field)) = payload.try_next().await {
        // let content_disposition = field.content_disposition().unwrap();
        // let filename = content_disposition.get_filename().unwrap_or("audio.wav");

        // Construct the file path for saving the file
        // file_path = format!("target/uploads/{}", filename);
        file_path = format!("target/uploads/audio.wav");
        // Open the file once for writing
        let f = web::block({
            let file_path = file_path.clone();
            move || File::create(&file_path)
        })
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))??;

        // Write each chunk of data to the file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            web::block({
                let mut f = f.try_clone().expect("Failed to clone file handle");
                move || f.write_all(&data)
            })
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))??;
        }
    }

    Ok(file_path)
}

/// This is our whisper model.
/// takes in the converted wav file, and the path to model, and after a number of
/// parameters and initialization, decodes the audio to text returning a result of
/// the transcribed text
pub fn whisper_transcribe_medium2() -> Result<String, Box<dyn std::error::Error>> {
    use hound;
    use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
    println!("Beginning whisper_transcribe_medium2 function :)");
    let audio_path: &str = "target/uploads/output_audio_code_convertor.wav";
    let path_to_model = "whisper.cpp/models/ggml-base.en.bin";

    // Initialize WhisperContext
    let ctx = WhisperContext::new_with_params(&path_to_model, WhisperContextParameters::default())?;

    // Set the number of threads in FullParams
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_n_threads(4); // Use 4 threads for transcription

    let mut reader = hound::WavReader::open(audio_path)?;
    let audio_data: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / i16::MAX as f32)
        .collect();

    let mut state = ctx.create_state()?;
    state.full(params, &audio_data)?;

    let mut transcription = String::new();
    for i in 0..state.full_n_segments()? {
        transcription.push_str(&state.full_get_segment_text(i)?);
    }
    println!("Transcription: {}", transcription);
    Ok(transcription)
}

/// This function converts the stored audio.wav file to output_audio_code_convertor.wav
/// file format in the target/uploads/ folder.
/// uses std::io::Command to use os apis.
pub fn wav_convert_file() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            "target/uploads/audio.wav",
            "-ar",
            "16000",
            "-ac",
            "1",
            "-sample_fmt",
            "s16",
            "target/uploads/output_audio_code_convertor.wav",
        ])
        .output()?;

    // Check if the command ran successfully
    if !output.status.success() {
        let stderr = str::from_utf8(&output.stderr)?;
        eprintln!("FFmpeg error: {}", stderr);
        return Err(Box::from(stderr));
    }

    Ok(String::from(
        "target/uploads/output_audio_code_convertor.wav",
    ))
}

// gb1.wav: RIFF (little-endian) data, WAVE audio, Microsoft PCM, 16 bit, mono 16000 Hz
// output.wav: RIFF (little-endian) data, WAVE audio, Microsoft PCM, 16 bit, mono 48000 Hz
