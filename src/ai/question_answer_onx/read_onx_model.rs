// #![allow(unused_imports)]
// use super::create_input_tensor::create_input_tensor;
// use super::extract_answer_from_output::extract_answer_from_output;
// use onnxruntime::environment::Environment;
// use onnxruntime::GraphOptimizationLevel;
// use onnxruntime::Result;

// #[allow(unused_variables)]
// pub fn ask_model_question(question: &str) -> Result<String> {
//     // Initialize the ONNX Runtime environment
//     let environment = Environment::builder().with_name("OnnxModel").build()?;

//     // Create the session with the correct ONNX model path and optimization settings
//     let session_builder = environment.new_session_builder()?;
//     let session = session_builder
//         .with_optimization_level(GraphOptimizationLevel::Basic)?
//         .with_model_from_file("onx_model/roberta_base_squad2.onnx")?;

//     // Additional logic to run the model and process input would go here
//     let tokenized_question = tokenize_question(question).expect("Failed to tokenize question");

//     // let input_tensor = create_input_tensor(tokenized_question)?;
//     // let result = session.run(vec![tokenized_question.into()])?;
//     // let answer = extract_answer_from_output(&tokenized_question)?;
//     let answer = "".to_string();
//     Ok(answer)
// }
