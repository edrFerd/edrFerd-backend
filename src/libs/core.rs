use crate::libs::data_struct::{Block, Chunk, ChunkData};
use crate::GLOBAL_SOCKET;
use chrono;
use log::{debug, error, info, warn};

// 工作循环
pub async fn receive_loop() -> anyhow::Result<()> {
    // 将套接字绑定到 "0.0.0.0:8080"，你可以根据需要更改端口
    let sock = GLOBAL_SOCKET.get().unwrap();
    info!("Listening on: {}", sock.local_addr()?);

    const BUF_SIZE: usize = 1024 * 1024;
    let mut buf = [0; BUF_SIZE]; // temp only

    loop {
        match sock.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                // 将接收到的字节转换为字符串
                if len > BUF_SIZE {
                    warn!("接受到了 > 1M 巨巨巨大包");
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
    // 检查时间差是否在2分钟（120秒）内
    let current_timestamp = chrono::Utc::now().time();
    let chunk_time = chunk.data.timestamp;
    let time_diff = if chunk_time > current_timestamp {
        chunk_time - current_timestamp
    } else {
        current_timestamp - chunk_time
    };
    let two_minutes = chrono::Duration::minutes(2);
    if time_diff > two_minutes {
        warn!(
            "时间戳验证失败：chunk时间 {:?} 与当前时间 {:?} 相差超过2分钟",
            chunk_time, current_timestamp
        );
        return Ok(());
    }
    debug!("时间戳验证通过：时间差为 {:?}", time_diff);

    // 验证签名
    // 验证PoW
    Ok(())
}
pub async fn send_explanation(block: Block, difficult: u32) -> anyhow::Result<()> {
    // TODO hash
    // TODO Salt
    let chunk_data = ChunkData::new(
        "unimpled_hash".parse()?,
        block,
        "random_salt".parse()?,
    );
    let chunk = Chunk::new(chunk_data);
    let json_str: String = serde_json::to_string(&chunk)?;
    let socket = GLOBAL_SOCKET.get().unwrap();
    socket
        .send_to(json_str.as_bytes(), "255.255.255.255:8080")
        .await?;
    Ok(())
}
