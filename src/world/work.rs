use crate::chunk::Chunk;
use crate::core::receive::ChunkWithTime;
use log::{debug, info};
use tokio::sync::mpsc::UnboundedReceiver;

/// 工作的间隙，单位为毫秒
/// 目前是20tick/s (50ms)
const WORK_INTERVAL_MS: i64 = 50;

/// 异步工作循环，按固定时间间隔执行任务。
///
/// 每次循环会等待 `WORK_INTERVAL_MS` 毫秒后再继续。
async fn work_loop(receiver: UnboundedReceiver<ChunkWithTime>) {
    info!("启动工作循环，间隔: {} 毫秒", WORK_INTERVAL_MS);
    loop {
        let current_tick_time = chrono::Utc::now();
        // 计算上一次循环的时间: 当前时间减去工作间隔
        let last_tick_time = current_tick_time - chrono::Duration::milliseconds(WORK_INTERVAL_MS);
        let next_tick_time = last_tick_time + chrono::Duration::milliseconds(WORK_INTERVAL_MS);
        let work_handler = tokio::spawn(work(
            current_tick_time,
            last_tick_time,
            next_tick_time,
            &receiver,
        ));
        // 等待下一个循环间隔
        tokio::time::sleep(tokio::time::Duration::from_millis(WORK_INTERVAL_MS as u64)).await;
        debug!("完成一次工作循环");
        if !work_handler.is_finished() {
            work_handler.abort();
        }
    }
}
async fn work(
    current_tick_time: chrono::DateTime<chrono::Utc>,
    last_tick_time: chrono::DateTime<chrono::Utc>,
    next_tick_time: chrono::DateTime<chrono::Utc>,
    receiver: &UnboundedReceiver<ChunkWithTime>,
) {
    info!(
        "开始工作，当前Tick时间: {current_tick_time}, 上一Tick时间: {last_tick_time}, 下一Tick时间: {next_tick_time}"
    );
    let mut buffer = Vec::new();
    receiver.recv_many(&mut buffer, 0)
}

/// 处理单个数据块的逻辑。
///
/// 参数:
/// - `chunk`: 需要处理的 `Chunk` 实例
async fn handle_chunk(chunk: Chunk) {
    info!("开始处理数据块: {chunk:?}");
}
