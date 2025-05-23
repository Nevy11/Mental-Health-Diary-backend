use pyo3::prelude::*;
use pyo3::types::PyModule;

pub fn load_roberta_model() {
    Python::with_gil(|py| {
        // Get the 'sys' module to manipulate the sys.path
        let sys = PyModule::import_bound(py, "sys").expect("Failed to import sys");

        // Modify sys.path (a list of directories)
        let sys_path = sys.getattr("path").expect("Failed to get sys.path");

        // Insert the current directory into sys.path
        sys_path
            .call_method1("insert", (0, "."))
            .expect("Failed to add current directory to sys.path");

        // Now import the ai_model module using import
        let ai_module = PyModule::import_bound(py, "ai_model").expect("Failed to import ai_model");

        // Initialize the pipeline
        ai_module
            .getattr("initialize_pipeline")
            .expect("Failed to find initialize_pipeline function")
            .call0()
            .expect("Failed to initialize pipeline");

        // Example: Ask a question
        let question = "What is the capital of France?";
        let answer: String = ai_module
            .getattr("ask_question")
            .expect("Failed to find ask_question function")
            .call1((question,))
            .expect("Failed to ask question")
            .extract()
            .expect("Failed to extract answer");

        // Print the answer
        println!("Question: {}\nAnswer: {}", question, answer);
    });
}
