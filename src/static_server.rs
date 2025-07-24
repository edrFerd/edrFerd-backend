use axum::Router;
use axum::routing::get;
use std::net::SocketAddr;

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

async fn pubkey() -> &'static str {
    "pubkey"
}

async fn explain() -> &'static str {
    "explain"
}

async fn info() -> &'static str {
    "info"
}

async fn world() -> &'static str {
    "world"
}

async fn query() -> &'static str {
    "query"
}
