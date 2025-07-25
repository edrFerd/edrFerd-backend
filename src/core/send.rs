use blake3::Hash as BlakeHash;

use crate::GLOBAL_SOCKET;
use crate::chunk::{Chunk, ChunkData};
use crate::libs::data_struct::Block;
use crate::world::work::cmp_hash;

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
        .send_to(json_str.as_bytes(), "255.255.255.255:8080")
        .await?;
    // TODO 把自己发的包也丢到receiver里面
    Ok(())
}
