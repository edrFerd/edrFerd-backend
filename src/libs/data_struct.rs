use std::hash::Hash;

#[allow(dead_code)]
use crate::libs::key::get_key;
use blake3::Hash as BlakeHash;
use chrono::{NaiveTime, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use serde::{Deserialize, Serialize};

pub struct Chunk {
    sign: Signature,
    pow: BlakeHash,
    data: ChunkData,
}

impl Chunk {
    pub fn verify(&self) -> bool {
        let key = self.data.pubkey.clone();
        let hash = {
            let mut hasher = blake3::Hasher::new();
            hasher.update(self.pow.as_bytes());
            let jsoned = serde_json::to_string(&self.data).expect("wtf");
            hasher.update(jsoned.as_bytes());
            hasher.finalize()
        };
        key.verify_strict(hash.as_bytes(), &self.sign)
            .ok()
            .is_some()
    }
}

/// 一个块
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct ChunkData {
    /// 当前应用程序的版本
    version: String,
    /// 前一个hash
    prev_hash: BlakeHash,
    /// 对某个方块的解释
    explanation: Block,
    /// 当前的时间戳
    timestamp: NaiveTime,
    /// 公钥
    pubkey: VerifyingKey,
    /// 盐
    salt: String, //来自injective区块链
}

impl ChunkData {
    pub fn new(prev_hash: BlakeHash, explanation: Block, salt: String) -> Self {
        let pubkey = get_key().verifying_key();
        ChunkData {
            version: crate::VERSION.to_string(),
            prev_hash,
            explanation,
            timestamp: Utc::now().time(),
            pubkey,
            salt,
        }
    }
}

/// 要声明的方块
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct Block {
    point: Point,
    block_appearance: BlockAppearance,
}
/// 目标的节点
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}
/// 方块外观
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct BlockAppearance {
    type_id: String,
}
