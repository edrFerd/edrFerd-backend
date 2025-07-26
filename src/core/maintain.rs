use crate::core::send::send_explation_in_time;
use crate::libs::data_struct::{Block, BlockInfo, BlockPoint};
use foldhash::{HashMap, HashMapExt};
use tokio::sync::Mutex;

use std::sync::OnceLock;
use std::time::Duration;

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
pub fn get_maintain_blocks() -> &'static Mutex<HashMap<BlockPoint, MaintainBlock>> {
    MAINTAIN_BLOCKS.get_or_init(|| Mutex::new(HashMap::new()))
}
pub async fn add_new_maintain_block(point: BlockPoint, maintain_block: MaintainBlock) {
    let mut guard = get_maintain_blocks().lock().await;
    guard.insert(point, maintain_block);
}
pub async fn remove_maintain_block(point: BlockPoint) {
    let mut guard = get_maintain_blocks().lock().await;
    guard.remove(&point);
}
pub async fn maintain_send() {
    let maintain_blocks = get_maintain_blocks().lock().await;
    if maintain_blocks.is_empty() {
        return;
    }
    let mut send_jobs = Vec::with_capacity(maintain_blocks.len());
    for (pos, block) in maintain_blocks.iter() {
        let send_block = Block::new(pos.clone(), block.info.clone());
        let job = send_explation_in_time(send_block, Duration::from_millis(block.duration));
        send_jobs.push(job);
    }
    for job in send_jobs {
        job.await.unwrap();
    }
}
