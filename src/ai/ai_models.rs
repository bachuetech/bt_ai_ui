use std::collections::HashMap;

use bt_http_utils::{HttpClient, HttpResponse};
use bt_logger::log_error;
use serde::{Deserialize, Serialize};

use crate::config::ai_config::{AIConfig, InteractionType};

#[derive(Deserialize, Serialize)]
struct ModelList {
    models: Vec<OllamaModel>,
}

#[derive(Deserialize, Serialize)]
pub struct OllamaModel {
    pub name: String,
    model: String,
    modified_at: String,
    size: i64,
    digest: String,
    details: OllamaModelDetails,
}

#[derive(Deserialize, Serialize)]
struct OllamaModelDetails {
    parent_model: String,
    format: String,
    family: String,
    families: Vec<String>,
    parameter_size: String,
    quantization_level: String,
}

/// Call the different platforms available to retrieve the available models.
/// Returns a list of modesl as Json inside a HttpResponse object.
pub async fn get_available_models_http(
    ai_config: &AIConfig,
    http_client: &HttpClient,
) -> HttpResponse {
    let mut req_status: u16 = 0;
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut adjusted_models: Vec<OllamaModel> = Vec::new();

    for plfm in ai_config.get_platform_list() {
        let resp: HttpResponse;
        match http_client
            .get(
                ai_config
                    .get_url(plfm.to_string(), InteractionType::Models)
                    .as_str(),
            )
            .await
        {
            Ok(r) => resp = r,
            Err(e) => {log_error!( "get_available_models_http", "HTTP Error retriving models. ERROR: {}", e.to_string());
                        return     HttpResponse {
                            status_code: 500, 
                            header: http_client.get_default_headers(),
                            body: "".to_owned(),
                        }
                    },
        }

        if !resp.is_error() {
            req_status = resp.status_code;
            headers = resp.header;
            let models: ModelList = serde_json::from_str(&resp.body)
                .expect("Error getting models. JSON was not well-formatted");
            for mut m in models.models {
                m.name = format!("{}:{}", &plfm, m.name);
                adjusted_models.push(m);
            }
        } else {
            if req_status == 0 {
                req_status = resp.status_code;
                headers = resp.header;
            }
        }
    }

    let json_answer = serde_json::to_string(&ModelList {
        models: adjusted_models,
    })
    .expect("Error: AIClient::get_available_models_http. Cannot convert Models to Json");

    HttpResponse {
        status_code: req_status,
        header: headers,
        body: json_answer.to_owned(),
    }
}
