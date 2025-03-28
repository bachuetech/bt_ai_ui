///Bachuetech AI Chat User Interface

use bt_logger::{build_logger, LogLevel, LogTarget};
use app::web_service::app_server_start;

mod app;
mod ai;

#[tokio::main]
async fn main() {
    build_logger("BACHUETECH", "BT.AI.UI", LogLevel::VERBOSE, LogTarget::STD_ERROR );
    app_server_start(None).await;
}