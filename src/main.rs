///Bachuetech AI User Interface
///Sets up a logger, gets the application configuration, binds to a TCP listener, and starts the server.
/// 

use bt_logger::{build_logger, log_trace, LogLevel, LogTarget};
use app::ai_server::ai_server_start;

mod config;
///mod utils;
mod app;
mod ai;
mod process_exit_codes;


#[tokio::main]
async fn main() {
    build_logger("BACHUETECH", "BT.AI.UI", LogLevel::VERBOSE, LogTarget::STD_ERROR );
    log_trace!("main","Inside APP");
    ai_server_start().await;
}