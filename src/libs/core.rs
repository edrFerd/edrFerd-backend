#[allow(unused)]
use std::borrow::Cow;
use crate::libs::data_struct::{Block, Chunk, ChunkData, InitBroadcast};
use crate::GLOBAL_SOCKET;
use chrono;
use chrono::TimeDelta;
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
                process_pack(received_data);
            }
            Err(e) => {
                error!("接收到数据失败: {e}");
            }
        }
    }
}

fn process_pack(data: Cow<str>) {
    match serde_json::from_str::<serde_json::Value>(&data) {
        Ok(data) => {
            if let Ok(c) = serde_json::from_value::<Chunk>(data.clone()) {
                
            } else if let Ok(c) = serde_json::from_value::<InitBroadcast>(data.clone()) {

            }
        }
        Err(e) => {
            warn!("aaaaaaaaaaa???");
        }
    }
}

fn process_chuck(chunk: Chunk) -> anyhow::Result<()> {
    // 检查时间差是否在2分钟（120秒）内
    let current_timestamp = chrono::Utc::now().time();
    let chunk_time = chunk.data.timestamp;
    let time_diff = if chunk_time > current_timestamp {
        chunk_time - current_timestamp
    } else {
        current_timestamp - chunk_time
    };
    const TWO_MINUTES: TimeDelta = chrono::Duration::minutes(2);
    if time_diff > TWO_MINUTES {
        warn!(
            "时间戳验证失败：chunk时间 {:?} 与当前时间 {:?} 相差超过2分钟",
            chunk_time, current_timestamp
        );
        return Ok(());
    }
    debug!("时间戳验证通过：时间差为 {:?}", time_diff);

    // 验证签名
    let is_verify_available = chunk.verify_sign();
    if !is_verify_available {
        warn!("签名验证不通过");
        return Ok(());
    }
    // 验证PoW
    let is_pow_available = chunk.verify_pow();
    if !is_pow_available {
        warn!("pow验证不通过");
        return Ok(());
    }
    // 验证没问题，现在需要修改方块列表了

    Ok(())
}
pub async fn send_explanation(block: Block, difficult: u32) -> anyhow::Result<()> {
    // TODO hash
    // TODO Salt
    let chunk_data = ChunkData::new(
        "unimpled_hash".parse()?,
        block,
        "random_salt".parse()?,
        114514
    );
    let chunk = Chunk::new(chunk_data);
    let json_str: String = serde_json::to_string(&chunk)?;
    let socket = GLOBAL_SOCKET.get().unwrap();
    socket
        .send_to(json_str.as_bytes(), "255.255.255.255:8080")
        .await?;
    Ok(())
}
