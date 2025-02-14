use bt_http_utils::{ContentType, HttpClient, HttpResponse};
use bt_logger::{log_debug, log_error, log_trace, log_verbose, log_warning};
use serde::{Deserialize, Serialize};

use crate::ai::{message, tools_proxy::tool_proxy};

use super::{
    ai_tools::Tool,
    message::{Message, MessageRole},
};

#[derive(Serialize)]
struct AIChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AIChatResponse {
    model: String,
    created_at: String,
    message: Message,
    done_reason: String,
    done: bool,
    total_duration: u128,
    load_duration: u128,
    prompt_eval_count: u64,
    prompt_eval_duration: u128,
    eval_count: u64,
    eval_duration: u128,
}

#[derive(Serialize, Deserialize)]
struct AIChatBodyMessage {
    message: Message,
    context: Vec<Message>,
    done: bool,
}

pub async fn model_chat( ai_model: &String, role: MessageRole, message: &String, context: Vec<Message>, system: Option<String>, tool_list: Option<Vec<Tool>>, 
                        current_date: &str, current_time: &str, context_size: usize, http_client: &HttpClient, chat_url: String ) -> HttpResponse {
    log_trace!( "model_chat", "Ready to start chat role {:?}: {}", &role, &message );

    let mut initial_msg: Vec<Message> = Vec::new();
    if let Some(sys_msg) = system {
        initial_msg.push(Message::new(
            MessageRole::SYSTEM,
            format!(
                "{}. The current date is {} and the current time is {}",
                sys_msg, &current_date, &current_time
            ),
        ));
    }
    initial_msg.extend(context.clone()); //payload.context.clone());
    let user_message = Message::new(role, message.to_string());
    initial_msg.push(user_message.clone()); //Needed Later to build the context (history)
                                            // Convert the struct to a JSON string

    let ai_request = AIChatRequest {
        model: ai_model.to_owned(),
        messages: initial_msg.clone(),
        stream: false,
        tools: tool_list.clone(),
    };

    let json_string = serde_json::to_string(&ai_request).unwrap();
    log_verbose!("model_chat", "Request: {}", &json_string);

    let resp: HttpResponse; 
    match http_client
        .post(&chat_url, &json_string, ContentType::JSON)
        .await{
            Ok(r) => resp = r,
            Err(e) => {log_error!( "model_chat", "HTTP Error when reaching url {} for Prompt {}. ERROR: {}", &chat_url, &message, e.to_string() );
                                return HttpResponse {
                                        status_code: 500,
                                        header: http_client.get_default_headers(),
                                        body: format!("HTTP Error when reaching url {} for Prompt {}. ERROR: {}", &chat_url, &message, e.to_string() ),
                                }
                            },
        }


    if resp.is_error() {
        log_error!( "model_chat", "HTTP Error {} when reaching url {} for Prompt {}", resp.status_code, &chat_url, &message );
        return HttpResponse {
            status_code: resp.status_code,
            header: resp.header,
            body: resp.body,
        };
    }

    log_debug!("model_chat","Raw answer {:?}", resp);
    let res: AIChatResponse = serde_json::from_str(&resp.body).unwrap();
    log_verbose!("model_chat", "AI Answer Struct (Open HTTP Response to check for tool requests): {:?}", &res );

    let mut tool_response: Option<HttpResponse> = None;
    if res.message.get_role().clone() == message::MessageRole::ASSISTANT
        && res.message.get_content() == ""
    {
        log_verbose!("model_chat", "Ready to call tools?");
        if let Some(tools) = res.message.get_tools() {
            log_verbose!("model_chat", "Ready to call tools! List of Tools available");
            for t in tools {
                log_verbose!("model_chat", "Calling tool {:?}", t);
                let func_result = tool_proxy(t.get_function_name(), &t.get_arguments());
                let func_msg = match func_result {
                    Ok(value) => value,
                    Err(e) => e,
                };

                initial_msg.push(res.message.clone());

                log_verbose!("model_chat", "Ready to pass answer! {}", &func_msg);
                tool_response = Some(
                    Box::pin(model_chat(
                        ai_model,
                        MessageRole::TOOL,
                        &func_msg,
                        initial_msg.clone(),
                        None,
                        tool_list.clone(),
                        "",
                        "",
                        context_size,
                        http_client,
                        chat_url.clone(),
                    ))
                    .await,
                );

                log_verbose!("model_chat", "Back, with answer after tool response.");
            }
        } else {
            log_warning!("model_chat","Ready to call tools, due to Assistant msg with empty content, but no tools provided");
        }
    } else {
        log_verbose!("model_chat", "Assistance answer with no empty message");
    }

    let mut new_ctx: Vec<Message> = Vec::new();
    new_ctx.extend(context);
    if new_ctx.len() > context_size {
        let n = new_ctx.len() - context_size;
        new_ctx.drain(0..n);
    }

    new_ctx.push(user_message);
    new_ctx.push(res.message.clone());

    let answer_body: AIChatBodyMessage;
    if let Some(value) = &tool_response {
        log_debug!("model_chat","Building Response with Tool Response {:?}", &value);
        let tool_res: AIChatBodyMessage = serde_json::from_str(&value.body).unwrap();
        new_ctx.push(tool_res.message.clone());
        answer_body = AIChatBodyMessage {
            message: tool_res.message,
            context: new_ctx,
            done: tool_res.done,
        };
    } else {
        log_debug!("model_chat", "Building Response with When NO Tool Response");
        answer_body = AIChatBodyMessage {
            message: res.message,
            context: new_ctx,
            done: true,
        };
    }

    let resp_body_json = serde_json::to_string(&answer_body).unwrap();
    log_verbose!("model_chat", "Returning: {}", &resp_body_json);
    let final_response = tool_response.unwrap_or(resp.clone());

    HttpResponse {
        status_code: final_response.status_code,
        header: final_response.header,
        body: resp_body_json,
    }
}
