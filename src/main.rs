use crate::static_server::server;

mod core;
mod logger;
mod static_server;
mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    logger::init_logger();

    // 初始化密钥管理器

    server().await;
}
