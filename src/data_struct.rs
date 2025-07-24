use chrono::NaiveTime;

/// 一个块
pub struct Chunk {
    version: String,
    prev_hash: String,
    explanation: Vec<Block>,
    timestamp: NaiveTime,
    pubkey: PubKey,
    pow: Pow,
    sign: String,
}

pub struct Pow {

}

pub struct PubKey {
    key: String
}

pub struct Block {
    point: Point,
    block_appearance: BlockAppearance,
}

pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}
pub struct BlockAppearance {
    type_id: String,
}
