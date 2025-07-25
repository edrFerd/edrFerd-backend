use crate::chunk::Chunk;
use chrono::TimeDelta;
use log::{debug, error, info, trace, warn};
use std::sync::OnceLock;
use tokio::sync::mpsc::UnboundedSender;
use crate::core::message::NetworkMessage;

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

/// 处理接收到的网络消息。
///
/// 根据消息类型（Init 或 Chunk）进行不同的处理。
///
/// 参数：
/// - `message`: 从 P2P 网络接收到的消息
pub fn process_pack(message: NetworkMessage) {
    match message {
        NetworkMessage::Init(init_message) => {
            info!("收到网络初始化消息: {:?}", init_message);
            // TODO: 在这里处理节点发现逻辑，例如将其添加到已知节点列表
        }
        NetworkMessage::Chunk(chunk) => {
            info!("收到新的区块数据块");
            if let Err(e) = process_chuck(chunk) {
                warn!("处理区块失败: {}", e);
            }
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
            "时间戳验证失败：chunk时间 {chunk_time:?} 与当前时间 {current_timestamp:?} 相差超过2分钟"
        );
        return Ok(());
    }
    debug!("时间戳验证通过：时间差为 {time_diff:?}");

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
