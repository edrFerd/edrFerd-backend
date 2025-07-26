use crate::world::work::BlockUpdatePack;
use tokio::sync::{mpsc, oneshot};

pub mod api_server;
pub mod debug_server;
pub mod frontend_server;

/// debug server port
pub const DEBUG_PORT: u16 = 1417;
/// 前端 (unity 交互)
pub const FRONTEND_PORT: u16 = 1416;
/// api (向其他的客户端提供服务)
pub const API_PORT: u16 = 1415;

pub async fn start_all_server(
    recv: oneshot::Receiver<()>,
    work_event_recv: mpsc::UnboundedReceiver<BlockUpdatePack>,
) -> anyhow::Result<()> {
    let (debug_sender, debug_receiver) = oneshot::channel();
    let debug_waiter = tokio::spawn(debug_server::web_main(debug_receiver));
    let (frontend_sender, frontend_receiver) = oneshot::channel();
    let frontend_waiter = tokio::spawn(frontend_server::web_main(
        frontend_receiver,
        work_event_recv,
    ));
    let (api_sender, api_receiver) = oneshot::channel();
    let api_waiter = tokio::spawn(api_server::web_main(api_receiver));

    recv.await?;
    frontend_sender.send(()).ok();
    debug_sender.send(()).ok();
    api_sender.send(()).ok();
    frontend_waiter.await?;
    debug_waiter.await?;
    api_waiter.await?;
    Ok(())
}
