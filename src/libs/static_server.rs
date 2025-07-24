use axum::{Json, Router};
use axum::routing::get;
use std::net::SocketAddr;
use crate::libs::key::get_key;

pub async fn server() {
    let app = Router::new()
        .route("/pubkey", get(pubkey))
        .route("/explain", get(explain))
        .route("/info", get(info))
        .route("/world", get(world))
        .route("/query", get(query));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pubkey() -> Json<[u8; 32]>{
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
