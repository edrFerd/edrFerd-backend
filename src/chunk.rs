use std::fmt::{Display, Formatter};
use std::io::Write;

use crate::libs::{data_struct::Block, key::get_key};

use blake3::Hash as BlakeHash;
use chrono::{NaiveTime, Utc};
use ed25519_dalek::{Signature, VerifyingKey, ed25519::signature::SignerMut};
use serde::{Deserialize, Serialize};

/// 数据块结构，包含签名、工作量证明和数据内容。
///
/// 这是区块链中的基本数据单元，每个块都包含：
/// - 数字签名用于验证数据完整性
/// - 工作量证明用于防止垃圾数据
/// - 实际的数据内容
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Chunk {
    pub sign: Signature,
    pub pow: BlakeHash,
    pub data: ChunkData,
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}

impl Chunk {
    /// 验证数据块的数字签名是否有效。
    ///
    /// 使用数据块中的公钥验证签名的正确性。
    ///
    /// 返回值：`bool` 签名是否有效
    pub fn verify_sign(&self) -> bool {
        let key = self.data.pub_key.clone();
        let hash = Self::hash_data_for_sign(&self.pow, &self.data);
        key.verify_strict(hash.as_bytes(), &self.sign)
            .ok()
            .is_some()
    }

    /// 验证工作量证明是否正确。
    ///
    /// 检查存储的 PoW 哈希是否与重新计算的哈希匹配。
    ///
    /// 返回值：`bool` 工作量证明是否有效
    pub fn verify_pow(&self) -> bool {
        self.pow == self.data.pow()
    }

    /// 为签名生成数据哈希。
    ///
    /// 将工作量证明哈希和数据内容组合后生成用于签名的哈希值。
    ///
    /// 参数：
    /// - `pow`: 工作量证明哈希
    /// - `data`: 数据块内容
    ///
    /// 返回值：`BlakeHash` 用于签名的哈希值
    fn hash_data_for_sign(pow: &BlakeHash, data: &ChunkData) -> BlakeHash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(pow.as_bytes());
        let jsoned = serde_json::to_string(data).expect("wtf");
        hasher.update(jsoned.as_bytes());
        hasher.finalize()
    }

    /// 创建新的数据块。
    ///
    /// 根据提供的数据内容自动生成签名和工作量证明。
    ///
    /// 参数：
    /// - `data`: 数据块内容
    ///
    /// 返回值：`Self` 新创建的数据块
    pub fn new(data: ChunkData) -> Self {
        let mut key = get_key();
        let pow = data.pow();
        let hash = Self::hash_data_for_sign(&pow, &data);
        let sign = key.sign(hash.as_bytes());
        Chunk { sign, pow, data }
    }

    /// 从原始数据创建数据块。
    ///
    /// 使用提供的签名和工作量证明直接构造数据块，不进行重新计算。
    ///
    /// 参数：
    /// - `data`: 数据块内容
    /// - `pow`: 工作量证明哈希
    /// - `sign`: 数字签名
    ///
    /// 返回值：`Self` 构造的数据块
    pub fn new_from_raw(data: ChunkData, pow: BlakeHash, sign: Signature) -> Self {
        Self { sign, pow, data }
    }
}

/// 数据块的内容部分。
///
/// 包含版本信息、前一个哈希、区块解释、时间戳、公钥和盐值等。
/// 一个块
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChunkData {
    /// 当前应用程序的版本
    version: String,
    /// 前一个hash
    prev_hash: BlakeHash,
    /// 对某个方块的解释
    pub explanation: Block,
    /// 当前的时间戳
    pub timestamp: NaiveTime,
    /// 公钥
    pub pub_key: VerifyingKey,
    /// 盐
    salt_from_chunks: String, //来自injective区块链
    /// 真正的盐
    this_is_rand: u64,
}

impl ChunkData {
    /// 创建新的数据块内容。
    ///
    /// 使用当前时间戳和密钥自动填充相关字段。
    ///
    /// 参数：
    /// - `prev_hash`: 前一个区块的哈希值
    /// - `explanation`: 区块解释
    /// - `salt`: 来自区块链的盐值
    /// - `rand`: 随机数
    ///
    /// 返回值：`Self` 新创建的数据块内容
    pub fn new(prev_hash: BlakeHash, explanation: Block, salt: String, rand: u64) -> Self {
        let pubkey = get_key().verifying_key();
        ChunkData {
            version: crate::VERSION.to_string(),
            prev_hash,
            explanation,
            timestamp: Utc::now().time(),
            pub_key: pubkey,
            salt_from_chunks: salt,
            this_is_rand: rand,
        }
    }

    /// 计算工作量证明哈希。
    ///
    /// 将数据内容序列化为 JSON 后计算 Blake3 哈希值。
    ///
    /// 返回值：`BlakeHash` 工作量证明哈希
    pub fn pow(&self) -> BlakeHash {
        let jsoned = serde_json::to_string(&self).expect("wtf");
        let mut hasher = blake3::Hasher::new();
        hasher.write_all(jsoned.as_bytes());
        hasher.finalize()
    }
}
