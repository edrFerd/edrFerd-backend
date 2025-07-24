use crate::utils::key::get_pubkey;
use blake3::Hash;
use chrono::{NaiveTime, Utc};
use ed25519_dalek::VerifyingKey;

/// 一个块
pub struct Chunk {
    /// 当前应用程序的版本
    version: String,
    /// 前一个hash
    prev_hash: Hash,
    /// 对某个方块的解释
    explanation: Block,
    /// 当前的时间戳
    timestamp: NaiveTime,
    /// 公钥
    pubkey: VerifyingKey,
    /// 工作量证明
    pow: Pow,
    /// 签名
    sign: String,
}

impl Chunk {
    pub fn new(prev_hash: Hash, explanation: Block) -> Self {
        Chunk {
            version: crate::VERSION.to_string(),
            prev_hash,
            explanation,
            timestamp: Utc::now().time(),
            pubkey: get_pubkey().verifying_key(),
            pow: Pow {},
            sign: String::new(),
        }
    }
}
/// 工作量证明
pub struct Pow {}
/// 要声明的方块
pub struct Block {
    point: Point,
    block_appearance: BlockAppearance,
}
/// 目标的节点
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}
/// 方块外观
pub struct BlockAppearance {
    type_id: String,
}
