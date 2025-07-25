use crate::libs::data_struct::Chunk;
use log::{debug, info};

const WORK_INTERVAL: f64 = 1.0 / 20.0;

/// 异步工作循环，按固定时间间隔执行任务。
///
/// 每次循环会等待 `WORK_INTERVAL` 后再继续。
async fn work_loop() {
    info!("启动工作循环，间隔: {} 秒", WORK_INTERVAL);
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(WORK_INTERVAL)).await;
        debug!("完成一次工作循环");
    }
}

/// 处理单个数据块的逻辑。
///
/// 参数:
/// - `chunk`: 需要处理的 `Chunk` 实例
async fn handle_chunk(chunk: Chunk) {
    info!("开始处理数据块: {:?}", chunk);
}
