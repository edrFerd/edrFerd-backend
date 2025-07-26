pub mod work;

use crate::libs::data_struct::BlockPoint;
use crate::libs::data_struct::{Block, BlockInfo};
use foldhash::HashMapExt;
use log::info;
use serde::Serialize;
use std::sync::{LazyLock, OnceLock};
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedSender;

/// 世界地图类型，键为 `BlockPoint`，值为 `BlockInfo`。
type WorldMapType = foldhash::HashMap<BlockPoint, BlockInfo>;

/// 全局的世界结构，包含世界地图数据。
#[derive(Debug, Clone, Serialize)]
pub struct World {
    pub world: WorldMapType,
}

/// 全局的 `World` 实例，使用 `OnceLock` 实现懒初始化。
pub static GLOBAL_WORLD: OnceLock<Mutex<World>> = OnceLock::new();

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
    /// 在世界地图中设置或更新指定位置的方块信息。
    ///
    /// 参数:
    /// - `block_point`: 方块的坐标点
    /// - `block_info`: 方块的相关信息
    pub fn set_block(&mut self, block_point: BlockPoint, block_info: BlockInfo) {
        info!("已设置方块: 点 {:?}, 信息: {:?}", &block_point, &block_info);
        self.world.insert(block_point, block_info);
    }
    pub fn as_block(&self) -> Vec<Block> {
        self.world
            .iter()
            .map(|(point, block_info)| Block {
                point: point.clone(),
                block_appearance: block_info.clone(),
            })
            .collect()
    }
}

/// 获取全局唯一的 `World` 实例的互斥锁。
///
/// 首次调用时会初始化 `GLOBAL_WORLD`，并返回对其实例的静态引用。
///
/// 返回值：`&'static Mutex<World>` 全局世界实例的互斥锁引用
pub fn get_world() -> &'static Mutex<World> {
    info!("获取全局 World 实例");
    GLOBAL_WORLD.get_or_init(|| Mutex::new(World::new()))
}
