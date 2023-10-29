use knotter_api::run_server;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let is_test_mode = args.contains(&"--test-mode".to_string());
    
    run_server(is_test_mode).await
}
