use blake3::Hash as BlakeHash;
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::atomic::AtomicBool;
use std::sync::LazyLock;
use std::time::Duration;

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

/// 是否在等待 pong
pub static WAIT_PONG: AtomicBool = AtomicBool::new(true);

pub async fn send_init() -> anyhow::Result<()> {
    let pack = InitBroadcast::new(false, PORT);
    log::info!("准备发送初始化信息");
    broadcast_by_udp(&pack).await?;
    log::info!("成功发送 init");
    Ok(())
}

pub async fn broadcast_by_udp<T: serde::Serialize>(data: &T) -> anyhow::Result<()> {
    let socket = GLOBAL_SOCKET.get().unwrap();
    let msg = serde_json::to_string(data)?;
    socket.set_broadcast(true)?;

    socket
        .send_to(msg.as_bytes(), ("255.255.255.255", PORT))
        .await?;
    Ok(())
}

pub async fn get_salt_from_injective() -> BlakeHash {
    static DOWNLOAD_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
        reqwest::ClientBuilder::new().build().expect("faild to build reqwest client")
    });
    const INJECTIVE_URL: &str = "https://lcd.injective.network/cosmos/base/tendermint/v1beta1/blocks/latest";
    let mut hasher = blake3::Hasher::new();
    let str = match DOWNLOAD_CLIENT.get(INJECTIVE_URL).send().await {
        Ok(d) => {
            format!("{d:?}")
        }
        Err(e) => {
            e.to_string()
        }
    };
    hasher.update(str.as_bytes());
    hasher.finalize()
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
    broadcast_by_udp(&chunk).await?;
    Ok(())
}

pub async fn send_explation_in_time(block: Block, cost: Duration) -> anyhow::Result<()> {
    let mut seed = 0_u64;
    let start_tick = std::time::Instant::now();
    let last_hash = blake3::hash("nice hash".as_bytes());
    let mut smallest: blake3::Hash = blake3::Hash::from_bytes([255; 32]);
    let mut the_chunk = ChunkData::new(last_hash, block.clone(), "some aaaa".to_string(), seed);
    loop {
        the_chunk = ChunkData::new(last_hash, block.clone(), "some aaaa".to_string(), seed);
        let hash = the_chunk.pow();
        if cmp_hash(&hash, &smallest).is_le() {
            smallest = hash;
        }
        seed += 1;
        if start_tick.elapsed() >= cost {
            break;
        }
    }
    // 发送
    let chunk = Chunk::new(the_chunk);
    broadcast_by_udp(&chunk).await?;
    Ok(())
}
