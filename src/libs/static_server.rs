use anyhow::Result;
use axum::routing::get;
use axum::{Json, Router};

use std::net::SocketAddr;

use crate::libs::key::get_key;

pub async fn server() -> Result<()> {
    let app = Router::new()
        .route("/pubkey", get(pubkey))
        .route("/explain", get(explain))
        .route("/info", get(info))
        .route("/world", get(world))
        .route("/query", get(query));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn web_main() -> Result<tokio::task::JoinHandle<()>> {
    let task = tokio::spawn(server);

    Ok(task)

}

async fn pubkey() -> Json<[u8; 32]> {
    Json(get_key().verifying_key().to_bytes())
}

async fn explain() -> &'static str {
    "explain"
}

async fn info() -> &'static str {
    "我是奶龙"
}

async fn world() -> &'static str {
    "world"
}

async fn query() -> &'static str {
    "query"
}
