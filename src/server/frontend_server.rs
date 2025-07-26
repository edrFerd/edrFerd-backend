use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use crate::libs::key::get_key;
use crate::server::FRONTEND_PORT;
use crate::world::BlockWithPubKey;

use anyhow::Result;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use tokio::sync::{Mutex, mpsc, oneshot};
use tower_http::cors::CorsLayer;

use crate::world::work::BlockUpdatePack;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

async fn server(event_recv: mpsc::UnboundedReceiver<BlockUpdatePack>) -> anyhow::Result<()> {
    let cors = CorsLayer::very_permissive();
    let app: Router = Router::new()
        .route("/known_world_state", get(known_world_state))
        .route("/pubkey", get(get_pubkey))
        .route("/tick_update_vec", get(tick_update_vec))
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
    while let Some(block_update_pack) = recv.lock().await.recv().await {
        drop(block_update_pack);
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

pub async fn web_main(
    stop_receiver: oneshot::Receiver<()>,
    event_recv: mpsc::UnboundedReceiver<BlockUpdatePack>,
) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server(event_recv));
    stop_receiver.await?;
    Ok(task)
}
