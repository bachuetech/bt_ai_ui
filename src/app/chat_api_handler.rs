use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    response::{Json, Response},
};
use bt_logger::{log_trace, log_verbose};
use serde::Deserialize;
use crate::ai::message::Message;

use super::app_state::AppState;

#[derive(Deserialize)]
pub struct ChatClientRequest {
    current_time: String,
    current_date: String,
    model: String,
    prompt: String,
    context: Option<Vec<Message>>, //Is Optional!!
}

pub async fn chat_handler(State(ws_state): State<Arc<AppState>>, 
                            Json(payload): Json<ChatClientRequest>,) -> Response<Body> {
    log_trace!("chat_handler", "Prompt = {}", &payload.prompt);

    let context: Vec<Message>;
    if let Some(ctx) = payload.context{
        if ctx.len() > 0{
            context = ctx;
        }else{
            context = Vec::new();
        }
    } else {
        context = Vec::new();
    }

    let chat_resp = ws_state
        .ai_client
        .chat_compleation(
            &payload.model,
            &payload.prompt,
            context,
            &payload.current_date,
            &payload.current_time,
        )
        .await;

    log_verbose!("chat_handler", "Chat Client Answer {:?}", &chat_resp);

    Response::builder()
        .status(chat_resp.status_code) // Set the status code
        .header("Content-Type", "application/json") // Optional: set content type header
        .body(Body::from(chat_resp.body)) // Set the body
        .unwrap()
}
