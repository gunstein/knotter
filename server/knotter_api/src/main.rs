use knotter_api::run_server;
use std::env;
use log::{debug, error, log_enabled, info, Level};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Vec<String> = env::args().collect();
    debug!("args: {:?}", args);
    let is_test_mode = args.contains(&"--test-mode".to_string());
    

    run_server(is_test_mode).await
}
