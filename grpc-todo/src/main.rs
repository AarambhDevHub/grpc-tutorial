use dotenv::dotenv;
use server::run_server;

mod db;
mod server;
mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    run_server().await;
}
