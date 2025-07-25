use serde::{Deserialize, Serialize};
use crate::chunk::Chunk;
use crate::core::send::InitBroadcast;

/// 在 P2P 网络中传输的统一消息格式。
#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// 初始化消息，用于节点加入网络时广播自身信息。
    Init(InitBroadcast),
    /// 区块数据消息。
    Chunk(Chunk),
}
