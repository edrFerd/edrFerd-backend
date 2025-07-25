use crate::chunk::Chunk;
use crate::core::receive::ChunkWithTime;
use crate::libs::data_struct::BlockPoint;
use log::{debug, info, warn};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedReceiver;

/// 工作的间隙，单位为毫秒
/// 目前是20tick/s (50ms)
const WORK_INTERVAL_MS: i64 = 50;

/// 异步工作循环，按固定时间间隔执行任务。
///
/// 每次循环会等待 `WORK_INTERVAL_MS` 毫秒后再继续。
pub async fn work_loop(mut receiver: UnboundedReceiver<ChunkWithTime>) {
    info!("启动工作循环，间隔: {} 毫秒", WORK_INTERVAL_MS);
    let mut current_tick = chrono::Utc::now();
    let mut last_tick = current_tick - chrono::Duration::milliseconds(WORK_INTERVAL_MS);
    loop {
        let mut buf = Vec::new();
        receiver.recv_many(&mut buf, 0).await;
        last_tick = current_tick; // 移交上一次tick
        current_tick = chrono::Utc::now(); // 当前tick时间
        work(buf, current_tick, last_tick).await;

        // 等待下一个循环间隔
        tokio::time::sleep(tokio::time::Duration::from_millis(WORK_INTERVAL_MS as u64)).await;
        debug!("完成一次工作循环");
    }
}

struct block_infos {
    
}
type ChunkMap = HashMap<BlockPoint, block_infos>;
/// 工作函数，处理receiver中的数据
///
/// 参数:
/// - `chunks`: 数据块列表
/// - `current_tick`: 当前tick时间
/// - `last_tick`: 上一次tick时间  
async fn work(
    chunks: Vec<ChunkWithTime>,
    current_tick: chrono::DateTime<chrono::Utc>,
    last_tick: chrono::DateTime<chrono::Utc>,
) {
    for chunk_with_time in chunks {
        info!("从receiver接收到数据块，时间戳: {}", chunk_with_time.time);
        // 判断是否这个块的时间比上一个tick时间早，如果是的话，就drop it
        if (chunk_with_time.time < last_tick) {
            warn!("遇到一个超时的数据块，丢弃 {last_tick} ");
            continue;
        }
        // 剩下的都在合法时间内，可以正常处理
        let chunk = chunk_with_time.chunk;
    }

    info!("完成本次工作循环的数据处理");
}

/// 验证数据块是否合法
///
/// 参数:
/// - `chunk_with_time`: 带时间戳的数据块
///
/// 返回值: `bool` - 数据块是否合法
async fn is_chunk_valid(chunk_with_time: &ChunkWithTime) -> bool {
    // TODO: 这里实现具体的合法性判断逻辑
    // 目前先做简单的时间验证，可以后续扩展为调用process_chuck函数

    let current_time = chrono::Utc::now();
    let time_diff = current_time - chunk_with_time.time;

    // 检查时间戳是否在合理范围内（比如5分钟内）
    let max_age = chrono::Duration::minutes(5);

    if time_diff > max_age {
        warn!("数据块时间戳过旧: {} 秒前", time_diff.num_seconds());
        return false;
    }

    if time_diff < chrono::Duration::zero() {
        warn!("数据块时间戳来自未来: {} 秒后", (-time_diff).num_seconds());
        return false;
    }

    debug!("数据块时间验证通过，时间差: {} 秒", time_diff.num_seconds());
    true
}

/// 处理单个数据块的逻辑。
///
/// 参数:
/// - `chunk`: 需要处理的 `Chunk` 实例
async fn handle_chunk(chunk: Chunk) {
    info!("开始处理数据块: {chunk:?}");
}
