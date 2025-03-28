use std::sync::Arc;

use axum::{routing::{get, post}, Router};
use bt_core_config::app_config::AppConfig;
use bt_logger::{log_trace, log_verbose};
use tower_http::services::ServeDir;
use bt_http_server::{default_handler, fallback_root, server_start};
use crate::ai::ai_client::AICLient;

use super::{app_state::AppState, chat_api_handler::chat_handler, web_models::models_handler};

pub struct AIWebServer {
    app_configuration: AppConfig,
    state: Arc<AppState>,
}


impl AIWebServer {
    pub fn new(config: &AppConfig) -> Self {
        let aic = AICLient::new(config.get_environment());
        let shared_state = Arc::new(AppState { ai_client: aic });
        log_verbose!("new","AI Client and Shared State are ready");

        Self {
            app_configuration: config.clone(),
            state: shared_state,
        }
    }

    pub fn get_routes(&self) -> Router{
        log_verbose!("get_routes","API Route {}. APP Route {}",&self.app_configuration.get_api_path(),&self.app_configuration.get_app_path());
       Router::new()
        .route("/", get(default_handler)) //This is the default path and eventually fallback
        .nest(&self.app_configuration.get_api_path(), self.get_api_routes())
        //.nest_service(&self.app_configuration.get_app_path(), self.get_app_web_route())
        .nest_service(&self.app_configuration.get_app_path(), ServeDir::new(&self.app_configuration.get_file_app_dir()))
        //.route("/health", get(health_check_handler)); // Non-prefixed route
        .fallback(fallback_root)    // Catch-all for 404 errors
    }

    fn get_api_routes(&self) -> Router {
         log_trace!("get_api_routes","Geting Router with API Routes");
        Router::new()
            //.route("/generate",post(generate_handler)) //ToDo: Complete the generate/Prompt Option
            .route(
                self.app_configuration.get_end_point("models").as_str(),
                get(models_handler),
            ) //Models
            .route(
                self.app_configuration.get_end_point("chat").as_str(),
                post(chat_handler),
            ) //chat
            .with_state(self.state.clone())
    }
}

pub async fn app_server_start(running_env: Option<String>) {
    let app_configuration = AppConfig::new(running_env);
    let web_svr = AIWebServer::new(&app_configuration);
    server_start(&app_configuration, web_svr.get_routes()).await;
}
