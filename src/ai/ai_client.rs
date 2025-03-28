use bt_ai_core::{ai_config::{AIConfig, InteractionType}, ai_tools::{AIToolManager, Tool}, message::{Message, MessageRole}};
use bt_http_cli_conf::get_http_client_bool_config;
use bt_http_utils::{HttpClient, HttpResponse};
use bt_logger::log_verbose;
use bt_string_utils::get_first_of_split;

use crate::ai::chat_model::model_chat;
use super::ai_models::get_available_models_http;

const AI_CLIENT_CONFIG_YML: &str = "config/ai/ai-client-config.yml";
const AI_CLIENT_CONFIG_YML_ENV_VAR_NAME: &str = "BT_AICLIENT_CONFIGYMLFILE";
pub struct AICLient {
    ai_config: AIConfig,
    http_client: HttpClient,
    tool_mgr: AIToolManager,
}

impl AICLient {
    pub fn new(run_environment: String) -> Self {
        Self {
            ai_config: AIConfig::new(&run_environment),
            http_client: HttpClient::new(false, true, 
                get_http_client_bool_config(&run_environment, &AI_CLIENT_CONFIG_YML_ENV_VAR_NAME.to_owned(),
                                &AI_CLIENT_CONFIG_YML.to_owned() ) ),
            tool_mgr: AIToolManager::new(&run_environment),
        }
    }

    pub async fn chat_compleation(&self, ai_model: &String, chat_message: &String, chat_context: Vec<Message>, 
                                                current_date: &String, current_time: &String) -> HttpResponse{
                                                    
        let (platform, model_version) = get_first_of_split(ai_model.as_str(),":");
        let (model_id, m_version) =  get_first_of_split(&model_version,":");
        let model = self.get_model(&platform, &model_id, &m_version);

        log_verbose!("chat_compleation", "Platform {} and Model {:?}",&platform, &model);

        let http_resp = 
       model_chat(&model, MessageRole::USER, chat_message, chat_context,
            self.get_system_msg(&platform, &model_id), self.get_tools(&platform, &model_id), 
            current_date, current_time, 
            self.get_max_ctx_size(&platform), &self.http_client, 
            self.get_url(&platform, InteractionType::Chat)).await;

            HttpResponse{
                header: http_resp.header,
                status_code: http_resp.status_code,
                body: http_resp.body,
            }
    }

    pub async fn get_models(&self) -> HttpResponse{
        get_available_models_http(&self.ai_config, &self.http_client).await
    }

    pub fn get_tools(&self, platform_name: &String, model_id: &String) -> Option<Vec<Tool>> {
        self.tool_mgr.get_tools(platform_name, model_id)
    }

    pub fn get_model(&self, platform_name: &String, model_id: &String, model_version: &String) -> String {
        self.ai_config.get_model(platform_name, model_id, model_version)
    }

    pub fn get_system_msg(&self, platform_name: &String, model_id: &String) -> Option<String> {
        self.ai_config.get_system_msg(platform_name, model_id)
    }

    pub fn get_max_ctx_size(&self, platform_name: &String) -> usize {
        self.ai_config.get_max_ctx_size(platform_name)
    }

    pub fn get_url(&self, platform_name: &String, interaction: InteractionType) -> String {
        self.ai_config.get_url(platform_name.to_string(), interaction)
    }
}
