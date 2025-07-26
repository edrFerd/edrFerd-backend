// 维护模块：管理维护区块的添加、移除和定时发送逻辑
use crate::core::send::send_explation_in_time;
use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use foldhash::{HashMap, HashMapExt};
use tokio::sync::Mutex;
use log::{info, debug};
use std::sync::OnceLock;
use std::time::Duration;

/// 维护区块结构，保存发送间隔和区块信息
#[derive(Clone)]
pub struct MaintainBlock {
    // 花费的时间(ms)
    duration: u64,
    info: BlockInfo,
}

static MAINTAIN_BLOCKS: OnceLock<Mutex<HashMap<BlockPoint, MaintainBlock>>> = OnceLock::new();

impl MaintainBlock {
    pub(crate) fn new(difficulty: u64, info: BlockInfo) -> Self {
        MaintainBlock {
            duration: difficulty,
            info,
        }
    }
}

/// 获取全局唯一的维护区块映射，返回一个异步互斥锁保护的 HashMap
pub fn get_maintain_blocks() -> &'static Mutex<HashMap<BlockPoint, MaintainBlock>> {
    MAINTAIN_BLOCKS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 将新的维护区块加入队列
pub async fn add_new_maintain_block(point: BlockPoint, maintain_block: MaintainBlock) {
    let mut guard = get_maintain_blocks().lock().await;
    info!("添加维护区块: {:?}, 发送延迟: {} ms", point, maintain_block.duration);
    guard.insert(point, maintain_block);
}

/// 从队列中移除指定的维护区块
pub async fn remove_maintain_block(point: BlockPoint) {
    let mut guard = get_maintain_blocks().lock().await;
    info!("移除维护区块: {:?}", point);
    guard.remove(&point);
}

/// 遍历当前所有维护区块，根据各自延迟定时发送区块更新消息
pub async fn maintain_send() {
    let maintain_blocks = get_maintain_blocks().lock().await;
    info!("开始执行维护发送，维护区块数量: {}", maintain_blocks.len());
    if maintain_blocks.is_empty() {
        debug!("无维护区块，跳过发送");
        return;
    }
    let mut send_jobs = Vec::with_capacity(maintain_blocks.len());
    for (pos, block) in maintain_blocks.iter() {
        debug!("调度发送任务 -> 区块: {:?}, 延迟 {} ms", pos, block.duration);
        let send_block = Block::new(pos.clone(), block.info.clone());
        let job = send_explation_in_time(send_block, Duration::from_millis(block.duration));
        send_jobs.push(job);
    }
    info!("等待所有发送任务完成");
    for job in send_jobs {
        job.await.unwrap();
    }
    info!("所有维护发送任务已完成");
}
