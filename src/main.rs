#![allow(unused)]

use clap::Parser;
use log::info;
use std::sync::{Arc, OnceLock};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// 命令行参数
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// 如果设置，则生成随机公钥而不是从文件读取
    #[arg(long)]
    pub random_key: bool,
}

/// 全局命令行参数实例
pub static ARGS: OnceLock<Cli> = OnceLock::new();

mod chunk;
mod core;
mod libs;
mod logger;
mod server;
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
    let cli = Cli::parse();
    ARGS.set(cli).expect("命令行参数设置失败");

    logger::init_logger();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
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
    let (server_send, server_recv) = oneshot::channel();

    // 创建数据处理channel
    let (chunk_sender, chunk_receiver) = mpsc::unbounded_channel();

    // 启动数据接收循环
    let receive_handle = tokio::spawn(core::receive::receive_loop(chunk_sender));
    log::info!("数据接收循环已启动");

    let (work_sender, work_receiver) = mpsc::unbounded_channel();
    // 启动数据处理工作循环
    let work_handle = tokio::spawn(world::work::work_loop(chunk_receiver, work_sender));
    log::info!("数据处理工作循环已启动");
    // 发送初始化信息
    core::send::send_init().await?;

    let server_waiter = tokio::spawn(server::start_all_server(server_recv, work_receiver));
    tokio::signal::ctrl_c().await.ok();
    server_send.send(()).ok();

    // 优雅关闭各个任务
    receive_handle.abort();
    work_handle.abort();

    log::info!("服务关闭");
    Ok(())
}
