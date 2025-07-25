#![allow(unused)]
use log::info;
use std::sync::{Arc, OnceLock};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

mod chunk;
mod core;
mod libs;
mod logger;
mod world;

/// 服务版本号，通过环境变量 `CARGO_PKG_VERSION` 获取。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PORT: u16 = 1414;
/// 全局 UDP 套接字实例，使用 `OnceLock` 实现懒初始化。
static GLOBAL_SOCKET: OnceLock<Arc<UdpSocket>> = OnceLock::new();

/// 获取全局 UDP 套接字的引用。
///
/// 返回值：`&'static Arc<UdpSocket>` 全局套接字引用
pub fn get_socket() -> &'static Arc<UdpSocket> {
    GLOBAL_SOCKET.get().unwrap()
}

/// 程序主入口点，初始化日志系统并启动异步运行时。
///
/// 返回值：`anyhow::Result<()>` 执行结果
fn main() -> anyhow::Result<()> {
    logger::init_logger();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    info!("启动异步运行时");
    runtime.block_on(async_main_logic())?;

    Ok(())
}

/// 异步主函数，负责初始化网络服务和信号处理。
///
/// 创建 UDP 套接字，启动 Web 服务器，并等待 Ctrl+C 信号来优雅关闭服务。
///
/// 返回值：`anyhow::Result<()>` 执行结果
async fn async_main_logic() -> anyhow::Result<()> {
    info!("服务启动");
    let socket = UdpSocket::bind(format!("0.0.0.0:{PORT}")).await?;
    socket.set_broadcast(true);
    GLOBAL_SOCKET.get_or_init(move || Arc::new(socket));
    let (send, recv) = oneshot::channel();

    // 创建数据处理channel
    let (chunk_sender, chunk_receiver) = mpsc::unbounded_channel();

    // 启动数据接收循环
    let receive_handle = tokio::spawn(core::receive::receive_loop(chunk_sender));
    log::info!("数据接收循环已启动");

    // 启动数据处理工作循环
    let work_handle = tokio::spawn(world::work::work_loop(chunk_receiver));
    log::info!("数据处理工作循环已启动");

    let waiter = tokio::spawn(libs::static_server::web_main(recv));

    tokio::signal::ctrl_c().await.ok();
    send.send(());

    // 优雅关闭各个任务
    receive_handle.abort();
    work_handle.abort();
    waiter.await;

    log::info!("服务关闭");
    Ok(())
}
