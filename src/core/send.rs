use blake3::Hash as BlakeHash;
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use if_addrs::{get_if_addrs, IfAddr};

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

/// 获取所有活跃网络接口的广播地址
fn get_broadcast_addresses() -> Vec<Ipv4Addr> {
    let mut addresses = Vec::new();

    if let Ok(interfaces) = get_if_addrs() {
        for interface in interfaces {
            if interface.is_loopback() {
                continue;
            }

            match interface.addr {
                IfAddr::V4(ipv4) => {
                    // 跳过无效网络接口
                    if ipv4.ip.is_unspecified() || ipv4.netmask.is_unspecified() {
                        continue;
                    }

                    let broadcast = calculate_broadcast(ipv4.ip, ipv4.netmask);
                    addresses.push(broadcast);
                }
                IfAddr::V6(_) => {} // 跳过IPv6接口
            }
        }
    }

    addresses
}

fn calculate_broadcast(ip: Ipv4Addr, netmask: Ipv4Addr) -> Ipv4Addr {
    let ip: u32 = ip.into();
    let netmask: u32 = netmask.into();
    Ipv4Addr::from(ip | !netmask)
}

pub async fn send_init() -> anyhow::Result<()> {
    let pack = InitBroadcast::new(false, PORT);
    let msg = serde_json::to_string(&pack)?;
    let socket = GLOBAL_SOCKET.get().unwrap();
    log::info!("准备发送初始化信息");
    socket.set_broadcast(true)?;

    // socket
    //     .send_to(msg.as_bytes(), ("255.255.255.255", PORT))
    //     .await?; //这个消息发送后自己也能收到。

    // 获取所有广播地址
    let broadcast_addresses = get_broadcast_addresses();

    if broadcast_addresses.is_empty() {
        log::warn!("未找到有效的广播地址，使用受限广播");
        let target = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), PORT);
        socket.send_to(msg.as_bytes(), target).await?;
    } else {
        for broadcast_addr in broadcast_addresses {
            let target = SocketAddrV4::new(broadcast_addr, PORT);
            match socket.send_to(msg.as_bytes(), target).await {
                Ok(_) => log::info!("成功发送到 {broadcast_addr}:{PORT}"),
                Err(e) => log::error!("发送到 {broadcast_addr} 失败: {e}"),
            }
        }
    }
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
