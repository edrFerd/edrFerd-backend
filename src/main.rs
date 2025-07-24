use libs::core::work_loop;
use libs::static_server::server;
use tokio::join;

mod libs;
mod logger;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    logger::init_logger();
    log::info!("服务启动");
    let (work_loop_result, _) = join!(work_loop(), server());
    if let Err(e) = work_loop_result {
        log::error!("退出的时候出现了错误{e}");
    }

    log::info!("服务关闭");
}
