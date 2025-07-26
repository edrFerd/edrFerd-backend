use crate::core::send::send_explanation;
use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use anyhow::Result;
use axum::extract::Query;
use axum::routing::{get, post};
use axum::{Json, Router};
use blake3::Hash as BlakeHash;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::oneshot::Receiver;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
pub struct BlockWithTime {
    block: [i64; 3],
    cost: u64,
}

async fn server() -> Result<()> {
    let cors = CorsLayer::very_permissive();

    let app = Router::new()
        .route("/test_send", get(test_send))
        .route("/show_world", get(show_world))
        .route("/send_block", post(send_block_from_web))
        .route("/send_block_with_time", post(send_block_with_time))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 1415));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("正在监听 {addr}作为服务器");
    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn show_world() -> Json<HashMap<String, BlockInfo>> {
    info!("触发 show_world");
    let world = crate::world::get_world().lock().await;
    let mut serializable_world = HashMap::new();
    for (point, info) in world.world.iter() {
        serializable_world.insert(point.to_string(), info.clone());
    }
    Json(serializable_world)
}

pub async fn test_send() -> String {
    info!("触发 test_send");
    let block = Block {
        point: BlockPoint::new(1, 2, 3),
        block_appearance: BlockInfo::new("test_block".to_string()),
    };
    let difficult: BlakeHash = blake3::hash(b"test difficulty");
    match send_explanation(block, difficult).await {
        Ok(_) => {
            let msg = "成功发送共识包".to_string();
            info!("{msg}");
            msg
        }
        Err(e) => {
            let error_msg = format!("发送共识包失败: {e:?}");
            log::error!("{error_msg}");
            error_msg
        }
    }
}

pub async fn send_block_from_web(Json(block): Json<Block>) -> String {
    info!("触发 send_block_from_web with block: {block:?}");
    let difficult: BlakeHash = blake3::hash(b"test difficulty");
    match send_explanation(block, difficult).await {
        Ok(_) => {
            let msg = "成功发送自定义共识包".to_string();
            info!("{msg}");
            msg
        }
        Err(e) => {
            let error_msg = format!("发送共识包失败: {e:?}");
            log::error!("{error_msg}");
            error_msg
        }
    }
}

pub async fn send_block_with_time(Json(data): Json<BlockWithTime>) -> String {
    info!("触发 send_block_with_time with data: {data:?}");
    
    // 根据指定的时间计算难度
    // 这里需要根据实际的 POW 算法来计算合适的难度值
    // 暂时使用一个简单的计算方式
    let difficulty_data = format!("difficulty_for_{}ms", data.cost).into_bytes();
    let difficult: BlakeHash = blake3::hash(&difficulty_data);
    
    let block = Block {
        point: BlockPoint::new(data.block[0], data.block[1], data.block[2]),
        block_appearance: BlockInfo::new("timed_block".to_string()),
    };
    
    match send_explanation(block, difficult).await {
        Ok(_) => {
            let msg = format!("成功发送耗时 {} 毫秒的 POW 方块", data.cost);
            info!("{msg}");
            msg
        }
        Err(e) => {
            let error_msg = format!("发送耗时 POW 方块失败: {e:?}");
            log::error!("{error_msg}");
            error_msg
        }
    }
}

pub async fn web_main(stop_receiver: Receiver<()>) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server());
    stop_receiver.await?;
    Ok(task)
}
