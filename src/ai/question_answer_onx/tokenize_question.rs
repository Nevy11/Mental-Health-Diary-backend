// use std::error::Error;
// use tokenizers::Tokenizer;

// // Function to tokenize the question and context using a tokenizer
// fn tokenize_input(question: &str, context: &str) -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error>> {
//     // Load the tokenizer (path to the tokenizer configuration file)
//     let tokenizer = Tokenizer::from_file("path/to/roberta-base-tokenizer.json")?;

//     // Tokenize the question and context (encode pairs of input)
//     let encoding = tokenizer.encode_pair(question, context, true)?;

//     // Get token IDs and attention mask
//     let input_ids = encoding.get_ids().to_vec();
//     let attention_mask = encoding.get_attention_mask().to_vec();

//     Ok((input_ids, attention_mask))
// }
