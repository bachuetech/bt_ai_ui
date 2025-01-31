use std::sync::Arc;

use axum::{body::Body, extract::State, response::Response};

use super::app_state::AppState;

//use super::web_app::AppState;

pub async fn models_handler(State(ws_state): State<Arc<AppState>>) -> Response<Body>{
    let resp = ws_state.ai_client.get_models().await;
    Response::builder()
    .status(resp.status_code) // Set the status code
    .header("Content-Type", "application/json") // set content type header
    .body(Body::from(resp.body)) // Set the body
    .unwrap() // Handle potential errors (for simplicity, unwrap here only)
}  