use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use crate::libs::key::get_key;
use crate::world::BlockWithPubKey;
use anyhow::Result;
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::oneshot::Receiver;
use tower_http::cors::CorsLayer;

async fn server() -> anyhow::Result<()> {
    let cors = CorsLayer::very_permissive();
    let app: Router = Router::new()
        .route("/known_world_state", get(known_world_state))
        .route("/pubkey", get(get_pubkey))
        .layer(cors);
    let addr = SocketAddr::from(([0, 0, 0, 0], 1416));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("正在监听 {addr}作为前端服务器");
    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn known_world_state() -> Json<Vec<BlockWithPubKey>> {
    info!("触发 known_world_state");
    let world = crate::world::get_world().lock().await;
    world.as_block_with_pub_key().into()
}

pub async fn get_pubkey() -> Json<Vec<u8>> {
    info!("触发 get_pubkey");
    // 将公钥序列化为字节数组
    let bytes = get_key().verifying_key().to_bytes();
    Json(bytes.to_vec())
}

pub async fn web_main(stop_receiver: Receiver<()>) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server());
    stop_receiver.await?;
    Ok(task)
}
