use crate::chunk::Chunk;
use crate::core::receive::ChunkWithTime;
use log::{debug, info, warn};
use tokio::sync::mpsc::UnboundedReceiver;

/// 工作的间隙，单位为毫秒
/// 目前是20tick/s (50ms)
const WORK_INTERVAL_MS: i64 = 50;

/// 异步工作循环，按固定时间间隔执行任务。
///
/// 每次循环会等待 `WORK_INTERVAL_MS` 毫秒后再继续。
pub async fn work_loop(mut receiver: UnboundedReceiver<ChunkWithTime>) {
    info!("启动工作循环，间隔: {} 毫秒", WORK_INTERVAL_MS);
    loop {
        let current_tick_time = chrono::Utc::now();
        // 计算上一次循环的时间: 当前时间减去工作间隔
        let last_tick_time = current_tick_time - chrono::Duration::milliseconds(WORK_INTERVAL_MS);
        let next_tick_time = last_tick_time + chrono::Duration::milliseconds(WORK_INTERVAL_MS);
        
        // 直接在这里处理数据，不再spawn新任务
        work(
            current_tick_time,
            last_tick_time,
            next_tick_time,
            &mut receiver,
        ).await;
        
        // 等待下一个循环间隔
        tokio::time::sleep(tokio::time::Duration::from_millis(WORK_INTERVAL_MS as u64)).await;
        debug!("完成一次工作循环");
    }
}

/// 工作函数，处理receiver中的数据
///
/// 参数:
/// - `current_tick_time`: 当前tick时间
/// - `last_tick_time`: 上一次tick时间  
/// - `next_tick_time`: 下一次tick时间
/// - `receiver`: 数据接收器的可变引用
async fn work(
    current_tick_time: chrono::DateTime<chrono::Utc>,
    last_tick_time: chrono::DateTime<chrono::Utc>,
    next_tick_time: chrono::DateTime<chrono::Utc>,
    receiver: &mut UnboundedReceiver<ChunkWithTime>,
) {
    info!(
        "开始工作，当前Tick时间: {current_tick_time}, 上一Tick时间: {last_tick_time}, 下一Tick时间: {next_tick_time}"
    );

    // 持续从receiver读取数据，直到遇到不合法数据或receiver为空
    loop {
        // 尝试从receiver中获取数据（非阻塞）
        match receiver.try_recv() {
            Ok(chunk_with_time) => {
                info!("从receiver接收到数据块，时间戳: {}", chunk_with_time.time);
                
                // 判断数据是否合法（目前使用简单的时间验证，后续可扩展）
                if is_chunk_valid(&chunk_with_time).await {
                    info!("数据块验证通过，开始处理");
                    // 合法数据，取出并处理
                    handle_chunk(chunk_with_time.chunk).await;
                } else {
                    warn!("数据块验证失败，停止处理并跳出循环");
                    // 不合法，停止处理，跳出循环
                    break;
                }
            }
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                debug!("receiver为空，跳出循环");
                // receiver为空，跳出循环
                break;
            }
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                warn!("receiver已断开连接，跳出循环");
                // receiver已断开，跳出循环
                break;
            }
        }
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
