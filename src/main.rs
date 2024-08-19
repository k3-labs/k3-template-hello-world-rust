use k3_wasm_macros::{define_k3_write_inputs, http_handler};
use k3_wasm_sdk::http::{Request, Response};
use serde_json::*;

#[http_handler]
pub fn get(_req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    // Create a JSON object
    let response_json = json!({
        "response": "Hello world from K3"
    });

    // Convert JSON object to a string and then to bytes
    let response_body = response_json.to_string().into_bytes();

    // Build the response
    Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(response_body)
        .unwrap()
}

k3_wasm_macros::init!();
