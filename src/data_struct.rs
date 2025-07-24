pub struct Chunk {
    version: String,
    prev_hash: String,
    explanation: Vec<Block>,
    timestamp: u128,
    pubkey: String,
    pow: String,
    sign: String,
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
