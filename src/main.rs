use crate::static_server::server;

mod core;
mod data_struct;
mod logger;
mod static_server;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    logger::init_logger();
    server().await;
}
