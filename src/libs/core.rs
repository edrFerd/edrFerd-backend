use crate::GLOBAL_SOCKET;
use crate::libs::data_struct::{Block, Chunk, ChunkData};
use chrono;
use log::{error, info, warn};
use serde_json::{Value, json};
use tokio::net::UdpSocket;

// 工作循环
pub async fn receive_loop() -> anyhow::Result<()> {
    // 将套接字绑定到 "0.0.0.0:8080"，你可以根据需要更改端口
    let sock = GLOBAL_SOCKET.get().unwrap();
    info!("Listening on: {}", sock.local_addr()?);

    const buf_size: usize = 1024 * 1024;
    let mut buf = [0; buf_size]; // temp only

    loop {
        match sock.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                // 将接收到的字节转换为字符串
                if len > buf_size {
                    warn!("接受到了 > 1M 巨包巨巨大包");
                    continue;
                };
                let received_data = String::from_utf8_lossy(&buf[..len]);
                info!("从 {addr} 接收到数据: {received_data}");
                // if let Ok(parsed_json) = serde_json::from_str::<Value>(&received_data) {
                //     info!("接收到有效的 JSON 数据: {parsed_json}");
                // } else {
                //     log::warn!(
                //         "接收到一个不能被反序列化为 json 的 udp 包\n来自: {addr} len: {len} e:{e}"
                //     );
                //     continue;
                // }
            }
            Err(e) => {
                error!("接收到数据失败: {e}");
            }
        }
    }
}
fn handle_receive_pack(chunk: Chunk) -> anyhow::Result<()> {
    let current_timestamp = chrono::Utc::now().timestamp();
    let chunk_time = chunk.data;
    // 判断时间容差，超过2分钟就丢弃这个包

    Ok(())
}
pub async fn send_explanation(block: Block, difficult: u32) -> anyhow::Result<()> {
    // TODO hash
    // TODO Salt
    let chunk_data = ChunkData::new(
        "unimpled_hash".parse().unwrap(),
        block,
        "random_salt".parse().unwrap(),
    );
    let chunk = Chunk::new(chunk_data);
    let json_str: String = serde_json::to_string(&chunk).unwrap();
    let socket = GLOBAL_SOCKET.get().unwrap();
    socket
        .send_to(json_str.as_bytes(), "255.255.255.255:8080")
        .await?;
    Ok(())
}
