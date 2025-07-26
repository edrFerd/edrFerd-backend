use std::{
    fmt::{Display, Formatter},
    hash::Hash,
    io::Write,
};

#[allow(dead_code)]
use crate::libs::key::get_key;
use blake3::Hash as BlakeHash;
use serde::{Deserialize, Serialize};

/// 区块信息，包含位置和外观数据。
/// 要声明的方块
#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct Block {
    pub point: BlockPoint,
    pub block_info: BlockInfo,
}
impl Block{
    pub fn new(point: BlockPoint, block_info: BlockInfo) -> Self {
        Self { point, block_info }
    }
}

/// 三维空间中的区块位置坐标。
/// 目标的节点
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct BlockPoint {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for BlockPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl BlockPoint {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        BlockPoint { x, y, z }
    }
}

/// 区块的外观信息，包含类型标识符。
/// 方块所属的信息
#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialOrd, PartialEq, Eq)]
pub struct BlockInfo {
    pub type_id: String,
}
impl BlockInfo {
    pub fn new(type_id: String) -> Self {
        Self { type_id }
    }
}
