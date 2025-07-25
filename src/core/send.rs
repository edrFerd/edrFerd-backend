use blake3::Hash as BlakeHash;
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use std::net::{Ipv4Addr, SocketAddrV4};

use crate::chunk::{Chunk, ChunkData};
use crate::libs::data_struct::Block;
use crate::libs::key::get_key;
use crate::world::work::cmp_hash;
use crate::{GLOBAL_SOCKET, PORT};

/// 初始化广播消息结构。
#[derive(Debug, Hash, Deserialize, Serialize)]
pub struct InitBroadcast {
    pub linten_only: bool,
    pub serve_port: u16,
    pub pub_key: VerifyingKey,
}

impl InitBroadcast {
    pub fn new(listen_only: bool, serve_port: u16) -> Self {
        let pub_key = get_key().verifying_key();
        Self {
            linten_only: listen_only,
            serve_port,
            pub_key,
        }
    }
}

pub async fn send_init() -> anyhow::Result<()> {
    let pack = InitBroadcast::new(false, PORT);
    let msg = serde_json::to_string(&pack)?;
    let socket = GLOBAL_SOCKET.get().unwrap();
    log::info!("准备发送初始化信息");
    socket.set_broadcast(true)?;

    socket
        .send_to(msg.as_bytes(), ("255.255.255.255", PORT))
        .await?;
    log::info!("成功发送 init");
    Ok(())
}

/// 发送区块解释到网络。
///
/// 创建一个包含指定区块和难度的数据块，并通过广播发送到网络。
///
/// 参数：
/// - `block`: 要发送的区块数据
/// - `difficult`: 挖矿难度
///
/// 返回值：`anyhow::Result<()>` 发送结果
pub async fn send_explanation(block: Block, difficult: BlakeHash) -> anyhow::Result<()> {
    let mut rand = 0_u64;
    loop {
        let one = ChunkData::new(difficult, block.clone(), "some aaaa".to_string(), rand);
        let hash = one.pow();
        if cmp_hash(&hash, &difficult).is_le() {
            break;
        }
        rand += 1;
    }
    let chunk_data = ChunkData::new(difficult, block, "some aaaa".to_string(), rand);
    let chunk = Chunk::new(chunk_data);
    let json_str: String = serde_json::to_string(&chunk)?;
    let socket = GLOBAL_SOCKET.get().unwrap();
    socket
        .send_to(json_str.as_bytes(), ("255.255.255.255", PORT))
        .await?;
    // TODO 把自己发的包也丢到receiver里面
    Ok(())
}
