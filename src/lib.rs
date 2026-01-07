use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;
use spin_sdk::llm;
use std::str;

// Define the model we want to use (must match spin.toml)
const MODEL: llm::InferencingModel = llm::InferencingModel::Llama2Chat;

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    // 1. Extract body from request
    let body = req.body();
    let input_text = str::from_utf8(body).unwrap_or("");

    if input_text.is_empty() {
        return Ok(Response::builder()
            .status(400)
            .body("Please provide text to analyze.")
            .build());
    }

    // 2. Construct the prompt for the AI
    let prompt = format!(
        "You are a sentiment analysis bot. 
         Analyze the following text and reply with ONE word: 'POSITIVE', 'NEGATIVE', or 'NEUTRAL'.
         Text: '{}'",
        input_text
    );

    // 3. Run Inference (This runs on the Node's CPU/GPU, not inside Wasm RAM)
    let inference_result = llm::infer(MODEL, &prompt);

    match inference_result {
        Ok(result) => {
            let sentiment = result.text.trim();
            println!("Input: {} | Sentiment: {}", input_text, sentiment);

            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(format!(r#"{{"sentiment": "{}"}}"#, sentiment))
                .build())
        },
        Err(e) => {
            eprintln!("AI Inference Error: {:?}", e);
            Ok(Response::builder()
                .status(500)
                .body("Internal AI Error")
                .build())
        }
    }
}