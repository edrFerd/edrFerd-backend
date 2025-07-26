use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use crate::libs::key::get_key;
use crate::server::API_PORT;
use crate::world::BlockWithPubKey;

use anyhow::Result;
use axum::routing::get;
use axum::{Json, Router};
use log::info;
use tokio::sync::oneshot::Receiver;
use tower_http::cors::CorsLayer;

use std::collections::HashMap;
use std::net::SocketAddr;

async fn server() -> anyhow::Result<()> {
    let cors = CorsLayer::very_permissive();
    let app: Router = Router::new()
        // .route("/known_world_state", get(known_world_state))
        // .route("/pubkey", get(get_pubkey))
        .layer(cors);
    let addr = SocketAddr::from(([0, 0, 0, 0], API_PORT));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("正在监听 {addr}作为前端服务器");
    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn web_main(stop_receiver: Receiver<()>) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server());
    stop_receiver.await?;
    Ok(task)
}
