// use onnxruntime::{environment::Environment, tensor::OrtOwnedTensor, GraphOptimizationLevel};
// use std::error::Error;
// use tokenizers::Tokenizer;

// fn ask_model_question() -> Result<(), Box<dyn Error + Send + Sync>> {
//     // Initialize the ONNX Runtime environment
//     let environment = Environment::builder().with_name("onnxruntime").build()?;

//     // Load the ONNX model into the session
//     let session = environment
//         .new_session_builder()?
//         .with_optimization_level(GraphOptimizationLevel::Basic)?
//         .with_model_from_file("path/to/roberta_base_squad2.onnx")?;

//     // Example question and context
//     let question = "Who was the first president of the United States?";
//     let context = "George Washington was the first president of the United States.";

//     // Tokenize the input question and context
//     let (input_ids, attention_mask) = tokenize_input(question, context)?;

//     // Convert input data to tensors
//     let input_tensor = OrtOwnedTensor::from_array(input_ids)?;
//     let attention_tensor = OrtOwnedTensor::from_array(attention_mask)?;

//     let result = session.run(vec![input_tensor, attention_tensor])?;

//     // Extract the answer from the model's output
//     let answer = extract_answer_from_output(&result, &input_ids)?;

//     println!("Answer: {}", answer);
//     Ok(())
// }

// // Function to tokenize the input question and context
// fn tokenize_input(
//     question: &str,
//     context: &str,
// ) -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error + Send + Sync>> {
//     // Load the tokenizer
//     let tokenizer = Tokenizer::from_file("path/to/roberta-base-tokenizer.json")?;

//     // Encode question and context
//     let encoding = tokenizer.encode(question.to_string() + " " + context, true)?;

//     // Extract the input IDs and attention mask
//     let input_ids = encoding
//         .get_ids()
//         .iter()
//         .map(|&id| id as i64)
//         .collect::<Vec<i64>>();
//     let attention_mask = encoding
//         .get_attention_mask()
//         .iter()
//         .map(|&mask| mask as i64)
//         .collect::<Vec<i64>>();

//     Ok((input_ids, attention_mask))
// }

// fn extract_answer_from_output<T>(
//     output: &[OrtOwnedTensor<f32, T>],
//     input_ids: &[i64],
// ) -> Result<String, Box<dyn Error + Send + Sync>>
// where
//     T: onnxruntime::ndarray::Dimension,
// {
//     let start_logits = &output[0];
//     let end_logits = &output[1];

//     let start_values = start_logits
//         .as_slice()
//         .ok_or("Failed to extract start logits")?;
//     let end_values = end_logits
//         .as_slice()
//         .ok_or("Failed to extract end logits")?;

//     let start_idx = start_values
//         .iter()
//         .enumerate()
//         .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
//         .map(|(idx, _)| idx)
//         .ok_or("Failed to find start index")?;

//     let end_idx = end_values
//         .iter()
//         .enumerate()
//         .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
//         .map(|(idx, _)| idx)
//         .ok_or("Failed to find end index")?;

//     if start_idx > end_idx {
//         return Err("Invalid start and end logits".into());
//     }

//     let answer_tokens = &input_ids[start_idx..=end_idx];
//     tokens_to_text(answer_tokens)
// }

// // Function to convert token IDs to readable text
// fn tokens_to_text(tokens: &[i64]) -> Result<String, Box<dyn Error + Send + Sync>> {
//     // Load the tokenizer to decode tokens
//     let tokenizer = Tokenizer::from_file("path/to/roberta-base-tokenizer.json")?;
//     let ids = tokens.iter().map(|&id| id as u32).collect::<Vec<u32>>();
//     let decoded = tokenizer.decode(&ids, true)?;
//     Ok(decoded)
// }
