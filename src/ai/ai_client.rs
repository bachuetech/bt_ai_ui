use bt_logger::log_verbose;

use crate::{ai::{chat_model::model_chat, message::MessageRole}, config::{self, ai_config::{AIConfig, InteractionType}}, utils::{http_utils::{HttpClient, HttpResponse}, string_utils::{get_first_ocurrance, get_first_of_split}}};

use super::{ai_models::get_available_models_http, ai_tools::{AIToolManager, Tool}, message::Message};

pub struct AICLient {
    ai_config: AIConfig,
    http_client: HttpClient,
    //chat_model: ChatModel,
    tool_mgr: AIToolManager,
}

impl AICLient {
    pub fn new(run_environment: String) -> Self {
        Self {
            ai_config: AIConfig::new(run_environment),
            http_client: HttpClient::new(),
            tool_mgr: AIToolManager::new(),
        }
    }

    pub async fn chat_compleation(&self, ai_model: &String, chat_message: &String, chat_context: Vec<Message>, 
                                                current_date: &String, current_time: &String) -> HttpResponse{
                                                    
        let (platform, model_version) = get_first_of_split(ai_model.as_str(),":");
        let model_id = get_first_ocurrance(&model_version, ":");
        let model = self.get_model(&platform, &model_id);

        log_verbose!("chat_compleation", "Platform {} and Model {:?}",&platform, &model);

        let http_resp = 
       model_chat(&model, MessageRole::USER, chat_message, chat_context,
            self.get_system_msg(&platform, &model_id), self.get_tools(&platform, &model_id), current_date, current_time, 
            self.get_max_ctx_size(&platform), &self.http_client, 
            self.get_url(&platform, config::ai_config::InteractionType::Chat)).await;

            HttpResponse{
                header: http_resp.header,
                status_code: http_resp.status_code,
                body: http_resp.body,
            }
    }

    pub async fn get_models(&self) -> HttpResponse{
        get_available_models_http(&self.ai_config, &self.http_client).await
    }

    pub fn get_model(&self, platform_name: &String, model_id: &String) -> String {
        if let Some(p) = self.ai_config.get_models((&platform_name).to_string()) {
            if let Some(model) = p.get(model_id) {
                model.model.clone()
            } else {
                model_id.clone()
            }
        } else {
            model_id.clone()
        }        
    }

    pub fn get_system_msg(&self, platform_name: &String, model_id: &String) -> Option<String> {
        if let Some(p) = self.ai_config.get_models((&platform_name).to_string()) {
            if let Some(sys) = p.get(model_id) {
                Some(format!("{}. {}", format!("Your Are {}", self.ai_config.get_name()), sys.system))
            } else {
                if model_id.to_lowercase() == "default" {
                    None
                } else {
                    self.get_system_msg(platform_name, &"default".to_owned())
                }
            }
        } else {
            None
        }
    }

    pub fn get_tools(&self, platform_name: &String, model_id: &String) -> Option<Vec<Tool>> {
        if let Some(p) = self.ai_config.get_models((&platform_name).to_string()) {
            if let Some(tool_model) = p.get(model_id) {
                if tool_model.tool_support{
                    self.tool_mgr.get_common_tools(tool_model.tools.clone())
                }else{
                    None
                }
            } else {
                if model_id.to_lowercase() == "default" {
                    None
                } else {
                    self.get_tools(platform_name, &"default".to_owned())
                }
            }
        } else {
            None
        }
    }

    pub fn get_max_ctx_size(&self, platform_name: &String) -> usize {
        self.ai_config.get_max_ctx_size(platform_name.to_string())
    }

    pub fn get_url(&self, platform_name: &String, interaction: InteractionType) -> String {
        self.ai_config.get_url(platform_name.to_string(), interaction)
    }
}
