use crate::static_server::server;

mod data_struct;
mod logger;
mod core;
mod static_server;

#[tokio::main]
async fn main() {
    logger::init_logger();
    server().await;
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
