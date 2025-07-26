use crate::libs::data_struct::BlockInfo;
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
        .layer(cors);
    let addr = SocketAddr::from(([0, 0, 0, 0], 1416));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("正在监听 {addr}作为前端服务器");
    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn known_world_state() -> Json<HashMap<String, BlockInfo>> {
    info!("触发 known_world_state");
    let world = crate::world::get_world().lock().await;
    let mut serializable_world = HashMap::new();
    for (point, info) in world.world.iter() {
        serializable_world.insert(point.to_string(), info.clone());
    }
    Json(serializable_world)
}

pub async fn web_main(stop_receiver: Receiver<()>) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server());
    stop_receiver.await?;
    Ok(task)
}
