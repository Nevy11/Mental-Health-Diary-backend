use tokenizers::Tokenizer;

use super::load_model::load_model_onx;

pub fn process_text(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Initialize the tokenizer
    let tokenizer = Tokenizer::from_file("distilbert-base-uncased-onnx/tokenizer.json").unwrap();

    // Tokenize the input
    let encoding = tokenizer.encode(input, true).unwrap();

    // Prepare input IDs and attention mask
    let input_id_vec: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
    let attention_mask_vec: Vec<i64> = encoding
        .get_attention_mask()
        .iter()
        .map(|&mask| mask as i64)
        .collect();

    // Call the model inference function with the tokenized inputs
    let decoded_output = load_model_onx(
        input_id_vec,
        attention_mask_vec,
        "distilbert-base-uncased-onnx/vocab.txt",
    )?;

    Ok(decoded_output)
}
