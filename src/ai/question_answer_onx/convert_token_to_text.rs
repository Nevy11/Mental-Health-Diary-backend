// Convert tokens back to human-readable text (you will need a tokenizer for this)
pub fn tokens_to_text(tokens: &[i64]) -> Result<String, Box<dyn std::error::Error>> {
    // Dummy implementation: In reality, you'd need a tokenizer to convert tokens back to words
    let decoded: Vec<String> = tokens.iter().map(|&token| token.to_string()).collect();
    Ok(decoded.join(" "))
}
