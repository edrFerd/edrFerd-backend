use libs::core::work_loop;
use libs::static_server::server;
use tokio::join;

mod logger;
mod libs;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    logger::init_logger();

    log::info!("服务启动");

    // 同时运行 work_loop 和 server
    // join! 宏会等待两个 future 都完成。
    // 由于这两个函数都是无限循环，所以 join! 会一直运行，直到程序被中断。
    let (work_loop_result, _) = join!(work_loop(), server());

    // 如果 work_loop 因为错误而退出，这里会打印错误信息。
    if let Err(e) = work_loop_result {
        log::error!("退出的时候出现了错误{}", e);
    }

    log::info!("服务关闭");
}
