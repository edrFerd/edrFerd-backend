use std::{hash::Hash, io::Write};

#[allow(dead_code)]
use crate::libs::key::get_key;
use blake3::Hash as BlakeHash;
use serde::{Deserialize, Serialize};

/// 区块信息，包含位置和外观数据。
/// 要声明的方块
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct Block {
    point: BlockPoint,
    block_appearance: BlockInfo,
}

/// 三维空间中的区块位置坐标。
/// 目标的节点
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct BlockPoint {
    x: i64,
    y: i64,
    z: i64,
}

/// 区块的外观信息，包含类型标识符。
/// 方块所属的信息
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct BlockInfo {
    type_id: String,
}

/// 初始化广播消息结构。
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct InitBroadcast {}
