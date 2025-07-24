use crate::libs::data_struct::{Block, Chunk, ChunkData};
use chrono;
use log::{error, info};
use serde_json::{Value, json};
use tokio::net::UdpSocket;

// 工作循环
pub async fn receive_loop() -> anyhow::Result<()> {
    // 将套接字绑定到 "0.0.0.0:8080"，你可以根据需要更改端口
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    info!("Listening on: {}", sock.local_addr()?);

    let mut buf = [0; 1024];

    loop {
        match sock.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                // 将接收到的字节转换为字符串
                let received_data = String::from_utf8_lossy(&buf[..len]);
                info!("从 {addr} 接收到数据: {received_data}");

                // 尝试将接收到的数据解析为 JSON
                match serde_json::from_str::<Value>(&received_data) {
                    Ok(parsed_json) => {
                        info!("接收到有效的 JSON 数据: {parsed_json}");
                    }
                    Err(e) => {
                        log::warn!(
                            "接收到一个不能被反序列化为 json 的 udp 包\n来自: {addr} len: {len}, e:{e}"
                        );
                        continue;
                    }
                };
            }
            Err(e) => {
                error!("接收到数据失败: {e}");
            }
        }
    }
}

pub fn send_explanation(block: Block, difficult: u32) {
    // TODO hash
    // TODO Salt
    let chunk_data = ChunkData::new(
        "unimpled_hash".parse().unwrap(),
        block,
        "random_salt".parse().unwrap(),
    );
    let chunk = Chunk::new(chunk_data);
    let json_str:String = serde_json::to_string(&chunk).unwrap();
    let socket = UdpSocket::bind("0.0.0.0:8080");
    
   
}
