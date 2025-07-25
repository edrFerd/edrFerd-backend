use crate::core::send::send_explanation;
use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use anyhow::Result;
use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use blake3::Hash as BlakeHash;
use log::info;
use std::net::SocketAddr;
use tokio::sync::oneshot::Receiver;

async fn server() -> Result<()> {
    let app = Router::new().route("/test_send", get(test_send));
    let addr = SocketAddr::from(([127, 0, 0, 1], 1414));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("正在监听 {addr}作为服务器");
    axum::serve(listener, app).await?;
    Ok(())
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
            info!("{}", msg);
            msg
        }
        Err(e) => {
            let error_msg = format!("发送共识包失败: {:?}", e);
            log::error!("{}", error_msg);
            error_msg
        }
    }
}

pub async fn web_main(stop_receiver: Receiver<()>) -> Result<tokio::task::JoinHandle<Result<()>>> {
    let task = tokio::spawn(server());
    stop_receiver.await?;
    Ok(task)
}
