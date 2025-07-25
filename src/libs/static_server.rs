use anyhow::Result;
use axum::routing::get;
use axum::{Json, Router};
use tokio::sync::oneshot::Receiver;

use std::net::SocketAddr;

use crate::libs::key::get_key;

/// 启动 HTTP 服务器。
///
/// 创建一个 Axum 路由器，配置各种 API 端点，并在本地地址上启动服务。
///
/// 返回值：`Result<()>` 服务器启动结果
async fn server() -> Result<()> {
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

/// Web 服务主入口点。
///
/// 启动 HTTP 服务器任务，并等待停止信号来优雅关闭服务。
///
/// 参数：
/// - `stop_reciver`: 用于接收停止信号的接收器
///
/// 返回值：`Result<tokio::task::JoinHandle<Result<()>>>` 服务器任务句柄
pub async fn web_main(stop_reciver: Receiver<()>) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server());
    stop_reciver.await?;
    Ok(task)
}

/// 获取公钥的 API 端点。
///
/// 返回当前节点的 Ed25519 公钥字节数组。
///
/// 返回值：`Json<[u8; 32]>` 公钥的 JSON 响应
async fn pubkey() -> Json<[u8; 32]> {
    Json(get_key().verifying_key().to_bytes())
}

/// 解释功能的 API 端点。
///
/// 返回值：`&'static str` 静态字符串响应
async fn explain() -> &'static str {
    "explain"
}

/// 节点信息的 API 端点。
///
/// 返回值：`&'static str` 节点描述信息
async fn info() -> &'static str {
    "我是奶龙"
}

/// 世界信息的 API 端点。
///
/// 返回值：`&'static str` 世界相关信息
async fn world() -> &'static str {
    "world"
}

/// 查询功能的 API 端点。
///
/// 返回值：`&'static str` 查询相关信息
async fn query() -> &'static str {
    "query"
}
