use super::convert_token_to_text::tokens_to_text;

// Extract answer from the model output
pub fn extract_answer_from_output(
    output: &[Vec<i64>],
) -> Result<String, Box<dyn std::error::Error>> {
    // Assuming the model returns a span (start and end indices for the answer)
    let start_idx = output[0][0] as usize; // Start token index
    let end_idx = output[0][1] as usize; // End token index

    // Convert the token indices back to text (this requires a mapping from token ID to word)
    let answer_tokens = &output[0][start_idx..=end_idx];

    // Example: Decode the answer from tokens (dummy implementation)
    let answer = tokens_to_text(answer_tokens)?;

    Ok(answer)
}
