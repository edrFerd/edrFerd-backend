mod work;

use crate::libs::data_struct::{BlockInfo, Chunk};
use crate::libs::data_struct::BlockPoint;
use foldhash::HashMapExt;
use std::sync::{LazyLock, OnceLock};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;

type WorldMapType = foldhash::HashMap<BlockPoint, BlockInfo>;

pub struct World {
    world: WorldMapType,
}

pub static WORLD_QUEUE: OnceLock<UnboundedSender<Chunk>> = OnceLock::new();

// 你说得对，但是原神是一款由米哈游开发的开放世界冒险游戏。在这款游戏中，你将进入一个名为提瓦特的幻想世界，
pub static GLOBAL_WORLD: OnceLock<World> = OnceLock::new();
impl World {
    pub fn new() -> Self {
        Self {
            world: WorldMapType::new(),
        }
    }
}
pub fn get_world() -> &'static World {
    GLOBAL_WORLD.get_or_init(World::new)
}
