use crate::libs::data_struct::BlockInfo;
use crate::libs::data_struct::BlockPoint;
use foldhash::HashMapExt;
use std::sync::OnceLock;

type WorldMapType = foldhash::HashMap<BlockPoint, BlockInfo>;
pub struct World {
    world: WorldMapType,
}
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
