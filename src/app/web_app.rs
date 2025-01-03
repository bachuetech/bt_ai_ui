use std::sync::Arc;


use axum::{
    http::Uri, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Router
};
use bt_logger::{log_debug, log_info, log_trace, log_verbose};
use tower_http::services::ServeDir;

use crate::{ai::ai_client::AICLient, config::app_config::AppConfig};

use super::{web_chat::chat_handler, web_models::models_handler};



//const APP_PATH: &str = "/app";

pub struct AIServer {
    app_configuration: AppConfig,
    state: Arc<AppState>,
}

pub struct AppState {
    pub ai_client: AICLient,
}

impl AIServer {
    pub fn new(config: AppConfig) -> Self {
        let app_config = config; 
        log_info!("new","Welcome to {} {}",app_config.get_app_name(),app_config.get_version());

        let aic = AICLient::new(app_config.get_environment());
        let shared_state = Arc::new(AppState { ai_client: aic });

        Self {
            app_configuration: app_config,
            state: shared_state,
        }
    }

    pub fn get_app_url(&self) -> String{
        self.app_configuration.get_app_path()
    }

    pub fn get_routes(&self) -> Router{
        log_verbose!("get_routes","API Route {}. APP Route {}",&self.app_configuration.get_api_path(),&self.app_configuration.get_app_path());
       Router::new()
        .route("/", get(handler)) //This is the default path and eventually fallback
        .nest(&self.app_configuration.get_api_path(), self.get_api_routes())
        //.nest_service(&self.app_configuration.get_app_path(), self.get_app_web_route())
        .nest_service(&self.app_configuration.get_app_path(), ServeDir::new(&self.app_configuration.get_file_app_dir()))
        //.route("/health", get(health_check_handler)); // Non-prefixed route
        .fallback(fallback)    // Catch-all for 404 errors
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

fn generate_html() -> String {
    format!("<h1>Bachutech AI</h1><br/><h2>Open http://localhost:3001/ai/app/</h2>" )
}

async fn handler() -> impl IntoResponse { //Redirect {
    log_trace!("handler","Default root.");
    let html_txt = generate_html(); 
    Html(html_txt)
}

async fn fallback(uri: Uri) -> impl IntoResponse {
    log_debug!("fallback", "Redirecting to default page. Page not found: {}", uri);
    Redirect::temporary("/")
}
