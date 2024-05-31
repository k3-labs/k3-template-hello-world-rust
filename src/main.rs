use k3_wasm_macros::http_handler;
use k3_wasm_sdk::http::{Request, Response};
use k3_wasm_sdk::{ENV_VAR_LEN, ENV_VAR_PTR};

#[http_handler]
pub fn get(_req: Request<Vec<u8>>) -> Response<Vec<u8>> {
    Response::builder()
        .status(200)
        .body(
            "Hello world from K3"
                .as_bytes()
                .to_vec(),
        )
        .unwrap()
}

k3_wasm_macros::init!();
