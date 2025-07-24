use std::{hash::Hash, io::Write};

#[allow(dead_code)]
use crate::libs::key::get_key;
use blake3::Hash as BlakeHash;
use chrono::{NaiveTime, Utc};
use ed25519_dalek::{Signature, VerifyingKey, ed25519::signature::SignerMut};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Chunk {
    pub sign: Signature,
    pub pow: BlakeHash,
    pub data: ChunkData,
}

impl Chunk {
    pub fn verify_sign(&self) -> bool {
        let key = self.data.pub_key.clone();
        let hash = Self::hash_data_for_sign(&self.pow, &self.data);
        key.verify_strict(hash.as_bytes(), &self.sign)
            .ok()
            .is_some()
    }
    pub fn verify_pow(&self) -> bool {
        self.pow == self.data.pow()
    }
    fn hash_data_for_sign(pow: &BlakeHash, data: &ChunkData) -> BlakeHash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(pow.as_bytes());
        let jsoned = serde_json::to_string(data).expect("wtf");
        hasher.update(jsoned.as_bytes());
        hasher.finalize()
    }

    pub fn new(data: ChunkData) -> Self {
        let mut key = get_key();
        let pow = data.pow();
        let hash = Self::hash_data_for_sign(&pow, &data);
        let sign = key.sign(hash.as_bytes());
        Chunk { sign, pow, data }
    }

    pub fn new_from_raw(data: ChunkData, pow: BlakeHash, sign: Signature) -> Self {
        Self { sign, pow, data }
    }
}

/// 一个块
#[derive(Debug, Deserialize, Serialize)]
pub struct ChunkData {
    /// 当前应用程序的版本
    version: String,
    /// 前一个hash
    prev_hash: BlakeHash,
    /// 对某个方块的解释
    explanation: Block,
    /// 当前的时间戳
    pub timestamp: NaiveTime,
    /// 公钥
    pub_key: VerifyingKey,
    /// 盐
    salt_from_chunks: String, //来自injective区块链
    /// 真正的盐
    this_is_rand: u64
}

impl ChunkData {
    pub fn new(prev_hash: BlakeHash, explanation: Block, salt: String, rand: u64) -> Self {
        let pubkey = get_key().verifying_key();
        ChunkData {
            version: crate::VERSION.to_string(),
            prev_hash,
            explanation,
            timestamp: Utc::now().time(),
            pub_key: pubkey,
            salt_from_chunks: salt,
            this_is_rand: rand
        }
    }

    pub fn pow(&self) -> BlakeHash {
        let jsoned = serde_json::to_string(&self).expect("wtf");
        let mut hasher = blake3::Hasher::new();
        hasher.write_all(jsoned.as_bytes());
        hasher.finalize()
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
