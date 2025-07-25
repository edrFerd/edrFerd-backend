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
