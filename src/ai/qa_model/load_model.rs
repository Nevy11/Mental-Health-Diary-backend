use onnxruntime::environment::Environment;
use onnxruntime::ndarray::Array2;
use onnxruntime::tensor::OrtOwnedTensor;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use tokenizers::Tokenizer;

// Load the vocabulary from a file
fn load_vocab(vocab_path: &str) -> Result<HashMap<i64, String>, Box<dyn Error>> {
    let mut vocab = HashMap::new();
    let file = File::open(vocab_path)?;
    let reader = io::BufReader::new(file);

    for (idx, line) in reader.lines().enumerate() {
        let line = line?.trim().to_string();
        vocab.insert(idx as i64, line);
    }

    Ok(vocab)
}

// Convert token ID to token
fn id_to_token(token_id: i64, vocab: &HashMap<i64, String>) -> String {
    vocab
        .get(&token_id)
        .cloned()
        .unwrap_or_else(|| "unknown".to_string())
}

// Function to pad or trim the sequence to the desired length
fn pad_or_trim_sequence(vec: &mut Vec<i64>, target_len: usize) {
    if vec.len() < target_len {
        vec.extend(vec![0; target_len - vec.len()]);
    } else if vec.len() > target_len {
        vec.truncate(target_len);
    }
}

// Load and run the ONNX model
pub fn load_model_onx(
    input_vec: Vec<i64>,
    attention_mask_vec: Vec<i64>,
    vocab_path: &str,
) -> Result<String, Box<dyn Error>> {
    let expected_length = 128;

    // Pad or trim input vectors
    let mut input_vec = input_vec.clone();
    let mut attention_mask_vec = attention_mask_vec.clone();
    pad_or_trim_sequence(&mut input_vec, expected_length);
    pad_or_trim_sequence(&mut attention_mask_vec, expected_length);

    // Initialize ONNX environment and session
    let environment = Environment::builder().with_name("bert-runtime").build()?;
    let mut session = environment
        .new_session_builder()?
        .with_model_from_file("distilbert-base-uncased-onnx/model.onnx")?; // Confirm model path

    // Prepare input tensors
    let input_ids = Array2::<i64>::from_shape_vec((1, expected_length), input_vec)?;
    let attention_mask = Array2::<i64>::from_shape_vec((1, expected_length), attention_mask_vec)?;

    // Run inference
    let outputs: Vec<OrtOwnedTensor<f32, _>> =
        session.run(vec![input_ids.into(), attention_mask.into()])?;

    // Decode output tensor
    let output_tensor = &outputs[0];
    let output_data = output_tensor.as_slice().unwrap();
    let mut predicted_token_ids = Vec::new();

    for token_logits in output_data.chunks(768) {
        let predicted_token_id = token_logits
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(index, _)| index as i64)
            .unwrap();
        predicted_token_ids.push(predicted_token_id);
    }

    // Load vocabulary and decode predictions
    let vocab = load_vocab(vocab_path)?;

    // Filter out unused tokens ([unused...]) from the predictions
    let decoded_output = predicted_token_ids
        .iter()
        .filter_map(|&id| {
            let token = id_to_token(id, &vocab);
            // Exclude tokens starting with [unused]
            if token.starts_with("[unused") {
                None
            } else {
                Some(token)
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("Decoded model output: {}", decoded_output);

    Ok(decoded_output)
}

// Decode output using the tokenizer
pub fn decode_output(output_ids: Vec<i64>, tokenizer: &Tokenizer) -> String {
    output_ids
        .iter()
        .filter_map(|&id| {
            // Exclude unused tokens during decoding
            let token = tokenizer
                .id_to_token(id as u32)
                .unwrap_or_else(|| "[UNK]".to_string());
            if token.starts_with("[unused") {
                None
            } else {
                Some(token)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
