mod work;

use crate::libs::data_struct::{BlockInfo, Chunk};
use crate::libs::data_struct::BlockPoint;
use foldhash::HashMapExt;
use std::sync::{LazyLock, OnceLock};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use log::info;

/// 世界地图类型，键为 `BlockPoint`，值为 `BlockInfo`。
type WorldMapType = foldhash::HashMap<BlockPoint, BlockInfo>;

/// 全局的世界结构，包含世界地图数据。
pub struct World {
    world: WorldMapType,
}

/// 全局的区块消息队列，用于在不同任务间传递 `Chunk`。
pub static WORLD_QUEUE: OnceLock<UnboundedSender<Chunk>> = OnceLock::new();

/// 全局的 `World` 实例，使用 `OnceLock` 实现懒初始化。
pub static GLOBAL_WORLD: OnceLock<World> = OnceLock::new();

impl World {
    /// 创建一个新的 `World` 实例，内部包含空的世界地图。
    ///
    /// 返回值：新创建的 `World` 实例
    pub fn new() -> Self {
        info!("初始化 World 实例");
        Self {
            world: WorldMapType::new(),
        }
    }
}

/// 获取全局唯一的 `World` 实例。
///
/// 首次调用时会初始化 `GLOBAL_WORLD`，并返回对其实例的静态引用。
///
/// 返回值：`&'static World` 全局世界实例引用
pub fn get_world() -> &'static World {
    info!("获取全局 World 实例");
    GLOBAL_WORLD.get_or_init(World::new)
}
