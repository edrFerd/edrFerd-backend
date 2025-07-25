use libp2p::{
    futures::StreamExt,
    gossipsub,
    mdns,
    swarm::{NetworkBehaviour, SwarmEvent},
    Swarm,
    PeerId,
    identity
};
use anyhow::Result;
use tokio::select;
use crate::core::message::NetworkMessage;
use std::sync::OnceLock;
use tokio::sync::mpsc;
use crate::core::receive::process_pack;
use log::{info, error, warn, debug, trace};

/// 用于向 P2P 事件循环发送消息的全局发送器。
static P2P_SENDER: OnceLock<mpsc::UnboundedSender<NetworkMessage>> = OnceLock::new();

/// 设置全局 P2P 发送器。
/// 此函数应在程序启动时由 `main` 函数调用一次。
pub fn set_p2p_sender(sender: mpsc::UnboundedSender<NetworkMessage>) {
    if P2P_SENDER.set(sender).is_err() {
        error!("P2P_SENDER has already been set.");
    }
}

/// 获取对全局 P2P 发送器的引用。
pub fn get_p2p_sender() -> &'static mpsc::UnboundedSender<NetworkMessage> {
    P2P_SENDER.get().expect("P2P_SENDER is not initialized")
}

/// 定义节点的网络行为
#[derive(NetworkBehaviour)]
#[behaviour(out_event = "P2pEvent")]
pub struct P2pBehaviour {
    /// 用于在局域网内发现其他对等节点
    pub mdns: mdns::tokio::Behaviour,
    /// 用于在网络中广播和接收消息
    pub gossipsub: gossipsub::Behaviour,
}

/// 从 P2pBehaviour 发送到 Swarm 的事件
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum P2pEvent {
    Mdns(mdns::Event),
    Gossipsub(gossipsub::Event),
}

impl From<mdns::Event> for P2pEvent {
    fn from(event: mdns::Event) -> Self {
        P2pEvent::Mdns(event)
    }
}

impl From<gossipsub::Event> for P2pEvent {
    fn from(event: gossipsub::Event) -> Self {
        P2pEvent::Gossipsub(event)
    }
}

/// 初始化 libp2p Swarm
pub async fn p2p_init() -> Result<Swarm<P2pBehaviour>> {
    // 创建一个新的 Ed25519 密钥对，用于节点身份认证
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    info!("本地节点 ID: {}", local_peer_id);

    // 构建 gossipsub 配置
    let gossipsub_config = gossipsub::Config::default();
    // 构建 gossipsub 行为
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    ).map_err(|e| anyhow::anyhow!(e))?;
    // 订阅一个示例主题
    let topic = gossipsub::IdentTopic::new("edrferd-topic");
    gossipsub.subscribe(&topic)?;

    // 构建 mdns 行为
    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?;

    // 将所有网络行为组合到 P2pBehaviour 中
    let behaviour = P2pBehaviour {
        gossipsub,
        mdns,
    };

    // 创建一个 Swarm
    let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_tcp(
            Default::default(),
            (libp2p::tls::Config::new, libp2p::noise::Config::new),
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_key| behaviour)?
        .with_swarm_config(|c| c.with_idle_connection_timeout(std::time::Duration::from_secs(60)))
        .build();

    // 监听一个由操作系统分配的地址和端口
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    Ok(swarm)
}

/// P2P 事件循环
pub async fn p2p_event_loop(
    mut swarm: Swarm<P2pBehaviour>,
    mut command_receiver: mpsc::UnboundedReceiver<NetworkMessage>,
) {
    loop {
        select! {
            // 处理从应用其他部分发来的命令
            Some(message) = command_receiver.recv() => {
                let topic = gossipsub::IdentTopic::new("ferdinand-blocks");
                match swarm.behaviour_mut().gossipsub.publish(topic.clone(), serde_json::to_string(&message).unwrap().as_bytes()) {
                    Ok(_) => trace!("成功广播消息"),
                    Err(e) => error!("广播消息失败: {:?}", e),
                }
            }
            // 处理来自 libp2p Swarm 的事件
            event = swarm.select_next_some() => {
                trace!("p2p event loop received event: {:?}", event);
                match event {
                    SwarmEvent::Behaviour(P2pEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list {
                            debug!("mDNS 发现新节点: {}", peer_id);
                            swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(P2pEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list {
                            debug!("mDNS 节点过期: {}", peer_id);
                        }
                    }
                    SwarmEvent::Behaviour(P2pEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: id,
                        message,
                    })) => {
                        debug!("收到来自 {} 的 gossipsub 消息: id={}", peer_id, id);
                        let msg: NetworkMessage = serde_json::from_slice(&message.data).unwrap();
                        process_pack(msg);
                    }
                    SwarmEvent::NewListenAddr { address, .. } => {
                        info!("p2p 节点正在监听: {}", address);
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        info!("已与节点建立连接: {}", peer_id);
                    }
                    SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                        warn!("与节点连接断开: {}, 原因: {:?}", peer_id, cause);
                    }
                    _ => {}
                }
            }
        }
    }
}
