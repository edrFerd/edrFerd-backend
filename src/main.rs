use std::sync::{Arc, OnceLock};
use tokio::net::UdpSocket;
use tokio::sync::oneshot;

mod libs;
mod logger;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

static GLOBAL_SOCKET: OnceLock<Arc<UdpSocket>> = OnceLock::new();

fn main() -> anyhow::Result<()> {
    logger::init_logger();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async_main())?;
    Ok(())
}

async fn async_main() -> anyhow::Result<()> {
    log::info!("服务启动");
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.set_broadcast(true)?;
    GLOBAL_SOCKET.get_or_init(move || Arc::new(socket));
    let (send, recv) = oneshot::channel();

    let waiter = tokio::spawn(libs::static_server::web_main(recv));

    tokio::signal::ctrl_c().await.ok();
    send.send(())?;

    waiter.await?;

    log::info!("服务关闭");
    Ok(())
}
