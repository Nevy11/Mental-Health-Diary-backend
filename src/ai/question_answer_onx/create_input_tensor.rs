pub fn create_input_tensor(
    tokenized_question: &[i64],
) -> Result<Vec<i64>, &Box<dyn std::error::Error>> {
    Ok(tokenized_question.to_vec())
}
