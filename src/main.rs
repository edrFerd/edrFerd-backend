use tokio::sync::oneshot;

mod libs;
mod logger;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    logger::init_logger();
    log::info!("服务启动");

    // let (work_loop_result, _) = join!(work_loop(), server());
    // if let Err(e) = work_loop_result {
    //     log::error!("退出的时候出现了错误{e}");
    // }

    let (send, recv) = oneshot::channel();

    let waiter = tokio::spawn(libs::static_server::web_main(recv));

    tokio::signal::ctrl_c().await.ok();
    send.send(());

    waiter.await;

    log::info!("服务关闭");
}
