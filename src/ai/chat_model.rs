use bt_ai_core::{ai_chat_helper::{get_chat_ai_chat_request, get_chat_request_json, AIChatBodyMessage, AIChatResponse}, ai_tools::Tool, message::{Message, MessageRole}};
use bt_http_utils::{ContentType, HttpClient, HttpResponse};
use bt_logger::{log_error, log_verbose, log_warning};

use crate::ai::tools_proxy::tool_proxy;

pub async fn model_chat( ai_model: &String, role: MessageRole, message: &String, context: Vec<Message>, system: Option<String>, tool_list: Option<Vec<Tool>>, 
                        current_date: &str, current_time: &str, context_size: usize, http_client: &HttpClient, chat_url: String ) -> HttpResponse {
    let ai_request = get_chat_ai_chat_request(ai_model, role.clone(), message, context.clone(), system, tool_list.clone(), current_date, current_time);
    let json_string = get_chat_request_json(&ai_request); //serde_json::to_string(&ai_request).unwrap();
    log_verbose!("model_chat", "Request: {}", &json_string);

    let resp: HttpResponse; 
    match http_client
        .post(&chat_url, None,&json_string, ContentType::JSON)
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

    //log_debug!("model_chat","Raw answer {:?}", resp);
    let res: AIChatResponse = serde_json::from_str(&resp.body).unwrap();
    log_verbose!("model_chat", "AI Answer Struct (Open HTTP Response to check for tool requests): {:?}", &res );

    let mut initial_msg = ai_request.messages.clone(); //Added 03/27/25

    let mut tool_response: Option<HttpResponse> = None;
    if res.message.get_role().clone() == MessageRole::ASSISTANT
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

    let user_message = Message::new(role, message.to_string()); //Added 03/27/25
    new_ctx.push(user_message);
    new_ctx.push(res.message.clone());

    let answer_body: AIChatBodyMessage;
    if let Some(value) = &tool_response {
        log_verbose!("model_chat","Building Response with Tool Response {:?}", &value);
        let tool_res: AIChatBodyMessage = serde_json::from_str(&value.body).unwrap();
        new_ctx.push(tool_res.message.clone());
        answer_body = AIChatBodyMessage {
            message: tool_res.message,
            context: new_ctx,
            done: tool_res.done,
        };
    } else {
        log_verbose!("model_chat", "Building Response When NO Tool Response");
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
