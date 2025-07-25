use crate::libs::data_struct::BlockPoint;
use crate::core::send::send_explanation;
use anyhow::Result;
use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use std::net::SocketAddr;
use tokio::sync::oneshot::Receiver;

async fn server() -> Result<()> {
    let app = Router::new().route("/",get(send));
    let addr = SocketAddr::from(([127, 0, 0, 1], 1415));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("正在监听 {addr}作为服务器");
    axum::serve(listener, app).await?;
    Ok(())
}
pub async fn test_send() {
    send_explanation(BlockPoint::new(0, 0), 0).await.unwrap();
}
pub async fn web_main(stop_receiver: Receiver<()>) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server());
    stop_receiver.await?;
    Ok(task)
}
