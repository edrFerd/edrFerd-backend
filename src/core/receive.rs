use crate::GLOBAL_SOCKET;
use crate::chunk::Chunk;
use crate::libs::data_struct::InitBroadcast;
use chrono::TimeDelta;
use log::{debug, error, info, trace, warn};
use std::borrow::Cow;
use std::sync::OnceLock;
use tokio::sync::mpsc::UnboundedSender;

pub struct ChunkWithTime {
    pub chunk: Chunk,
    pub time: chrono::DateTime<chrono::Utc>,
}

impl ChunkWithTime {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            time: chrono::Utc::now(),
        }
    }
}

/// 监听 UDP 套接字并处理接收到的数据包。
///
/// 本函数将套接字绑定到 "0.0.0.0:8080"，你可以根据需要更改端口。
/// 然后，它将进入一个无限循环，等待套接字接收到 UDP 数据包。
///
/// 在接收到数据包时，它将将其解析为 `Chunk` 或 `InitBroadcast` 类型，
/// 并根据解析结果进行相应处理。
///
/// 该函数会在发现解析错误时打印错误信息。
pub async fn receive_loop(sender: UnboundedSender<ChunkWithTime>) -> anyhow::Result<()> {
    // 将套接字绑定到 "0.0.0.0:8080"，你可以根据需要更改端口
    CHUNK_SENDER.get_or_init(|| sender);
    let sock = GLOBAL_SOCKET.get().unwrap();
    info!("Listening on: {}", sock.local_addr()?);

    const BUF_SIZE: usize = 1024 * 1024;
    let mut buf = vec![0; BUF_SIZE]; // temp only

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

/// 处理接收到的数据包。
///
/// 尝试将接收到的 JSON 数据解析为 `Chunk` 或 `InitBroadcast` 类型，
/// 并根据解析结果进行相应处理。
///
/// 参数：
/// - `data`: 接收到的字符串数据
fn process_pack(data: Cow<str>) {
    match serde_json::from_str::<serde_json::Value>(&data) {
        Ok(data) => {
            if let Ok(c) = serde_json::from_value::<Chunk>(data.clone()) {
                process_chuck(c);
            } else if let Ok(c) = serde_json::from_value::<InitBroadcast>(data.clone()) {
            }
        }
        Err(e) => {
            warn!("a?,{e}");
        }
    }
}

static CHUNK_SENDER: OnceLock<UnboundedSender<ChunkWithTime>> = OnceLock::new();

/// 处理单个数据块，包括时间戳验证、签名验证和工作量证明验证。
///
/// 该函数会依次验证：
/// 1. 时间戳是否在合理范围内（2分钟内）
/// 2. 数字签名是否有效
/// 3. 工作量证明是否正确
///
/// 参数：
/// - `chunk`: 要处理的数据块
///
/// 返回值：`anyhow::Result<()>` 处理结果
fn process_chuck(chunk: Chunk) -> anyhow::Result<()> {
    let sender = CHUNK_SENDER.get().expect("Sender未初始化");

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
        warn!("签名验证不通过,{chunk}");
        return Ok(());
    }
    // 验证PoW
    let is_pow_available = chunk.verify_pow();
    if !is_pow_available {
        warn!("pow验证不通过,{chunk}");
        return Ok(());
    }
    // 验证没问题，现在需要修改方块列表了
    // 丢进sender里面
    sender.send(ChunkWithTime::new(chunk))?;
    trace!("chunk已发送");
    Ok(())
}
