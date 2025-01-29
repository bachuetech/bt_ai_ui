use std::process;

use bt_logger::{log_fatal, log_info, log_verbose};
use tokio::signal;

use crate::{app::web_app::AIWebServer, config::{app_config::AppConfig, serv_config::get_srv_config}};

pub async fn ai_server_start() {
    let app_configuration = AppConfig::new();
    let srv_config = get_srv_config(app_configuration.get_environment()); 

    let tcp_binding_result = tokio::net::TcpListener::bind(srv_config.get_tcp_listener()).await;

    let listener = match tcp_binding_result {
        Ok(result) => result,
        Err(e) => {
            log_fatal!("main", "Fatal Error binding TCP {}. Error: {}", srv_config.get_tcp_listener(), e);
            process::exit(-100); // Exit the program with code -100
        }
    };

    log_verbose!("main", "listening on {}", listener.local_addr().unwrap());
    
    let svr = AIWebServer::new(app_configuration.clone());
    if srv_config.is_secure() {
        log_info!("main","To start open https://localhost:{}{}/",srv_config.get_port(),&svr.get_app_url());
    }else{
        log_info!("main","To start open http://localhost:{}{}/",srv_config.get_port(),&svr.get_app_url());
    }

    let server = axum::serve(listener, svr.get_routes()).with_graceful_shutdown(graceful_shutdown());
    
    if let Err(err) = server.await{
        log_fatal!("main","Web Server error: {}", err);
    }
}

// Graceful shutdown handler
async fn graceful_shutdown() {
    // Wait for a termination signal (Ctrl+C, SIGTERM, etc.)
    signal::ctrl_c().await.unwrap();
    log_info!("graceful_shutdown","Shutting down server...");
}