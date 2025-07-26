use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use crate::libs::key::get_key;
use crate::server::FRONTEND_PORT;
use crate::world::BlockWithPubKey;
// 33550336
use anyhow::Result;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use log::{info, trace};
use tokio::sync::{Mutex, mpsc, oneshot};
use tower_http::cors::CorsLayer;

use crate::world::work::BlockUpdatePack;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::core::maintain::{MaintainBlock, add_new_maintain_block, remove_maintain_block};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetBlockParams {
    duration: u64,
    x: i64,
    y: i64,
    z: i64,
    info: BlockInfo,
}

#[derive(Deserialize)]
pub struct RemoveBlockParams {
    x: i64,
    y: i64,
    z: i64,
}

async fn server(event_recv: mpsc::UnboundedReceiver<BlockUpdatePack>) -> anyhow::Result<()> {
    let cors = CorsLayer::very_permissive();
    let app: Router = Router::new()
        .route("/known_world_state", get(known_world_state))
        .route("/pubkey", get(get_pubkey))
        .route("/tick_update_vec", get(tick_update_vec))
        .route("/set_block", post(set_block))
        .route("/remove_block", post(remove_block))
        .with_state(Arc::new(Mutex::new(event_recv)))
        .layer(cors);
    let addr = SocketAddr::from(([0, 0, 0, 0], FRONTEND_PORT));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("正在监听 {addr}作为前端服务器");
    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn tick_update_vec(
    State(recv): State<Arc<Mutex<mpsc::UnboundedReceiver<BlockUpdatePack>>>>,
) -> Json<Vec<BlockUpdatePack>> {
    Json({
        let mut buf = Vec::new();
        let mut rec = recv.lock().await;
        if rec.is_empty() || rec.is_closed() {
            buf
        } else {
            while let Ok(block) = rec.try_recv() {
                buf.push(block);
            }
            buf
        }
    })
}

pub async fn known_world_state(
    State(recv): State<Arc<Mutex<mpsc::UnboundedReceiver<BlockUpdatePack>>>>,
) -> Json<Vec<BlockWithPubKey>> {
    info!("触发 known_world_state");
    while let Ok(block) = recv.lock().await.try_recv() {
        drop(block);
    }
    let world = crate::world::get_world().lock().await;
    world.as_block_with_pub_key().into()
}

pub async fn get_pubkey() -> Json<Vec<u8>> {
    info!("触发 get_pubkey");
    // 将公钥序列化为字节数组
    let bytes = get_key().verifying_key().to_bytes();
    Json(bytes.to_vec())
}

pub async fn set_block(Json(params): Json<SetBlockParams>) -> Json<&'static str> {
    add_new_maintain_block(
        BlockPoint::new(params.x, params.y, params.z),
        MaintainBlock::new(params.duration, params.info),
    )
    .await;
    Json("OK")
}

pub async fn remove_block(Json(params): Json<RemoveBlockParams>) -> Json<&'static str> {
    remove_maintain_block(BlockPoint::new(params.x, params.y, params.z)).await;
    Json("OK")
}

/// 启动一个前端服务器，用于处理来自前端的请求和响应。
///
/// 该函数异步执行，返回一个任务句柄，以便在其他任务中等待该任务的完成。
///
/// 该函数会在 `stop_receiver` 中接收到值时自动停止。
///
/// # Errors
///
/// 如果 `stop_receiver` 关闭或发生其他错误，返回 `Err`。
///
/// # Panics
///
/// 如果 `event_recv` 关闭或发生其他错误， panic。
pub async fn web_main(
    stop_receiver: oneshot::Receiver<()>,
    event_recv: mpsc::UnboundedReceiver<BlockUpdatePack>,
) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server(event_recv));
    stop_receiver.await?;
    Ok(task)
}
