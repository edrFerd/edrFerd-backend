use crate::chunk::Chunk;
use crate::core::receive::ChunkWithTime;
use crate::libs::data_struct::{BlockInfo, BlockPoint};
use crate::world::get_world;
use blake3::Hash as BlakeHash;
use ed25519_dalek::VerifyingKey;
use foldhash::HashMapExt;

use log::{debug, info, trace, warn};
use std::any::Any;
use std::cmp::Ordering;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedReceiver;

/// 工作的间隙，单位为毫秒
/// 目前是20tick/s (50ms)
const WORK_INTERVAL_MS: i64 = 50;

/// 异步工作循环，按固定时间间隔执行任务。
///
/// 每次循环会等待 `WORK_INTERVAL_MS` 毫秒后再继续。
pub async fn work_loop(mut receiver: UnboundedReceiver<ChunkWithTime>) {
    info!("启动工作循环，间隔: {WORK_INTERVAL_MS} 毫秒");
    let mut current_tick = chrono::Utc::now();
    let mut last_tick = current_tick - chrono::Duration::milliseconds(WORK_INTERVAL_MS);
    loop {
        let mut buf = Vec::new();
        while let Ok(chunk) = receiver.try_recv() {
            buf.push(chunk);
        }
        last_tick = current_tick; // 移交上一次tick
        current_tick = chrono::Utc::now(); // 当前tick时间
        work(buf, current_tick, last_tick).await;
        // 等待下一个循环间隔
        tokio::time::sleep(tokio::time::Duration::from_millis(WORK_INTERVAL_MS as u64)).await;
    }
}
#[derive(Debug, Hash, PartialEq, Eq)]
struct InfoKey {
    pub_key: VerifyingKey,
    pub block_appearance: BlockInfo,
}
impl InfoKey {
    pub fn new(pub_key: VerifyingKey, block_appearance: BlockInfo) -> Self {
        Self {
            pub_key,
            block_appearance,
        }
    }
}
type BlockInfoMap = foldhash::HashMap<InfoKey, BlakeHash>;
type ChunkMap = foldhash::HashMap<BlockPoint, BlockInfoMap>;

pub fn cmp_hash(hash1: &BlakeHash, hash2: &BlakeHash) -> Ordering {
    let h1 = hash1.as_bytes();
    let h2 = hash2.as_bytes();
    for (b1, b2) in h1.iter().zip(h2.iter()) {
        match b1.cmp(b2) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}

pub fn hash_add(hash1: &BlakeHash, hash2: &BlakeHash) -> BlakeHash {
    let h1 = hash1.as_bytes();
    let h2 = hash2.as_bytes();
    let mut need_next = false;
    let mut hash = [0u8; 32];
    for ((b1, b2), b3) in h1.iter().zip(h2.iter()).zip(hash.iter_mut()).rev() {
        let tmp = (*b1 as u32) + (*b2 as u32) + (if need_next { 1 } else { 0 });
        need_next = tmp > 255;
        *b3 = (tmp % 256) as u8;
    }
    BlakeHash::from_bytes(hash)
}

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
    if (chunks.is_empty()) {
        return;
    }
    let mut chunk_map = ChunkMap::new();
    for chunk_with_time in chunks {
        info!("从receiver接收到数据块，时间戳: {}", chunk_with_time.time);
        // 判断是否这个块的时间比上一个tick时间早，如果是的话，就drop it
        if (chunk_with_time.time < last_tick) {
            warn!("遇到一个超时的数据块，丢弃 {last_tick} ");
            continue;
        }
        // 剩下的都在合法时间内，可以正常处理
        let chunk = chunk_with_time.chunk;
        let chunk_block_point = chunk.data.explanation.point;
        let info_key = InfoKey::new(chunk.data.pub_key, chunk.data.explanation.block_appearance);
        let inner: &mut BlockInfoMap = chunk_map.entry(chunk_block_point).or_default();
        match inner.get_mut(&info_key) {
            Some(pow) => {
                *pow = hash_add(pow, &chunk.pow);
            }
            None => {
                inner.insert(info_key, chunk.pow);
            }
        }
        info!("已完成一个数据块的插入")
    }
    // 获取全局 World 实例的互斥锁并获取可变引用
    let world_mutex = get_world();
    let mut world = world_mutex.lock().await;
    for (point, info_map) in chunk_map {
        // 找到pow最大的entry
        if let Some((info_key, pow)) = info_map.into_iter().max_by(|(_, a), (_, b)| cmp_hash(a, b))
        {
            let appearance = &info_key.block_appearance;
            info!("Point {point:?} 最优方块外观: {appearance:?}, pow: {pow:?}");
            // 将更新后的方块写入到World中
            world.set_block(point, BlockInfo::new(appearance.type_id.clone()));
        }
    }
    info!("完成本次工作循环的数据处理");
}
