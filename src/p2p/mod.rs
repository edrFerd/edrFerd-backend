use libp2p::{
    futures::StreamExt,
    gossipsub::{self, IdentTopic as Topic, GossipsubEvent, MessageId},
    identity, mdns, noise,
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent, derive_prelude::*},
    tcp, yamux, Multiaddr, PeerId, Transport,
};
use tokio::sync::mpsc;

/// 业务侧发送到 P2P 服务的命令
#[derive(Debug, Clone)]
pub enum P2pCommand {
    /// 向指定 topic 广播数据
    Publish { topic: String, data: Vec<u8> },
}

/// P2P 服务向业务侧回传的事件
#[derive(Debug, Clone)]
pub enum P2pEvent {
    Message {
        from: PeerId,
        topic: String,
        data: Vec<u8>,
    },
    PeerDiscovered(PeerId),
}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "MyBehaviourEvent", event_process = false)]
pub struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

pub struct P2pService {
    swarm: libp2p::swarm::Swarm<MyBehaviour>,
    rx_cmd: mpsc::UnboundedReceiver<P2pCommand>,
    tx_evt: mpsc::UnboundedSender<P2pEvent>,
}

impl P2pService {
    pub async fn new(
        tx_evt: mpsc::UnboundedSender<P2pEvent>,
        rx_cmd: mpsc::UnboundedReceiver<P2pCommand>,
    ) -> anyhow::Result<Self> {
        // 生成本地密钥
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(local_key.public());
        log::info!("本地 peer id: {peer_id}");

        // Transport: TCP -> Noise -> Yamux
        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::NoiseAuthenticated::xx(&local_key).unwrap())
            .multiplex(yamux::YamuxConfig::default())
            .boxed();

        // gossipsub 配置
        let gs_config = gossipsub::ConfigBuilder::default()
            .message_id_fn(|m: &gossipsub::GossipsubMessage| MessageId::from(&m.data))
            .build()
            .expect("构建 gossipsub 配置失败");
        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gs_config,
        )?;

        // 默认订阅 broadcast topic
        let broadcast_topic = Topic::new("broadcast");
        gossipsub.subscribe(&broadcast_topic)?;

        // mdns 发现
        let mdns = mdns::tokio::Behaviour::default(peer_id)?;

        let behaviour = MyBehaviour { gossipsub, mdns };
        let mut swarm = SwarmBuilder::with_tokio_executor(transport, behaviour, peer_id).build();

        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse::<Multiaddr>()?)?;

        Ok(Self {
            swarm,
            rx_cmd,
            tx_evt: tx_evt.clone(),
        })
    }

    pub async fn run(mut self) {
        loop {
            tokio::select! {
                Some(cmd) = self.rx_cmd.recv() => match cmd {
                    P2pCommand::Publish { topic, data } => {
                        let topic = Topic::new(topic);
                        // 忽略错误
                        let _ = self.swarm.behaviour_mut().gossipsub.publish(topic, data);
                    }
                },
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message { propagation_source, message, .. })) => {
                        let _ = self.tx_evt.send(P2pEvent::Message {
                            from: propagation_source,
                            topic: message.topic.as_str().into(),
                            data: message.data,
                        });
                    },
                    SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer, _addr) in list {
                            let _ = self.tx_evt.send(P2pEvent::PeerDiscovered(peer));
                        }
                    },
                    SwarmEvent::NewListenAddr { address, .. } => {
                        log::info!("P2P listening on {address}");
                    },
                    _ => {}
                }
            }
        }
    }
}

/// 业务层简易广播器
pub struct Broadcaster {
    tx_cmd: mpsc::UnboundedSender<P2pCommand>,
}

impl Broadcaster {
    pub fn new(tx_cmd: mpsc::UnboundedSender<P2pCommand>) -> Self {
        Self { tx_cmd }
    }

    pub fn broadcast(&self, bytes: Vec<u8>) {
        let _ = self.tx_cmd.send(P2pCommand::Publish {
            topic: "broadcast".into(),
            data: bytes,
        });
    }
}