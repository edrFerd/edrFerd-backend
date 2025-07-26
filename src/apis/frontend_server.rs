use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use log::{info, warn, error};
use tokio::time;
use chrono;

// WebSocket 消息类型
#[derive(Clone, Debug)]
pub enum FrontendMessage {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

// 应用状态，用于管理 WebSocket 连接
#[derive(Clone)]
pub struct AppState {
    // 广播通道，用于向所有连接的客户端发送消息
    pub tx: broadcast::Sender<FrontendMessage>,
}

impl AppState {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        Self { tx }
    }
}

// 创建前端服务器路由
pub fn create_frontend_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/ws", get(websocket_handler))
}

// WebSocket 升级处理器
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

// 处理 WebSocket 连接
async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    info!("[WebSocket] 新客户端连接");
    
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();
    
    // 处理发送消息的任务
    let tx_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let result = match msg {
                FrontendMessage::Text(text) => {
                    sender.send(Message::Text(text.into())).await
                }
                FrontendMessage::Binary(data) => {
                    sender.send(Message::Binary(data.into())).await
                }
                FrontendMessage::Ping(data) => {
                    sender.send(Message::Ping(data.into())).await
                }
                FrontendMessage::Pong(data) => {
                    sender.send(Message::Pong(data.into())).await
                }
            };
            
            if result.is_err() {
                warn!("[WebSocket] 客户端连接断开");
                break;
            }
        }
    });
    
    // 处理接收消息的任务
    let rx_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    info!("[WebSocket] 收到消息: {}", text);
                    handle_text_message(text.to_string(), &state).await;
                }
                Ok(Message::Binary(data)) => {
                    info!("[WebSocket] 收到二进制数据 ({} bytes)", data.len());
                    handle_binary_message(data.to_vec(), &state).await;
                }
                Ok(Message::Ping(data)) => {
                    // Ping/Pong 是心跳消息，不需要记录日志
                    let _ = state.tx.send(FrontendMessage::Pong(data.to_vec()));
                }
                Ok(Message::Pong(_)) => {
                    // Pong 响应，不需要记录日志
                }
                Ok(Message::Close(_)) => {
                    info!("[WebSocket] 客户端主动关闭连接");
                    break;
                }
                Err(e) => {
                    error!("[WebSocket] 连接错误: {}", e);
                    break;
                }
            }
        }
    });
    
    // 等待任一任务完成
    tokio::select! {
        _ = tx_task => {}
        _ = rx_task => {}
    }
    
    info!("[WebSocket] 客户端连接已断开");
}

// 处理文本消息
async fn handle_text_message(text: String, state: &Arc<AppState>) {
    info!("[WebSocket] 处理文本消息: {}", text);
    
    // 示例：回显消息
    let response = format!("服务器收到: {}", text);
    let _ = state.tx.send(FrontendMessage::Text(response));
}

// 处理二进制消息
async fn handle_binary_message(data: Vec<u8>, state: &Arc<AppState>) {
    info!("[WebSocket] 处理二进制数据 ({} bytes)", data.len());
    
    // 示例：回显二进制数据
    let _ = state.tx.send(FrontendMessage::Binary(data));
}

// 启动前端服务器
pub async fn start_frontend_server(port: u16) -> anyhow::Result<()> {
    let state = Arc::new(AppState::new());
    
    let app = create_frontend_routes().with_state(state.clone());
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("[WebSocket] 前端 WebSocket 服务器启动在端口 {}", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}

// 用于向所有连接的客户端广播消息的辅助函数
pub fn broadcast_message(state: &Arc<AppState>, message: FrontendMessage) {
    let _ = state.tx.send(message);
}
