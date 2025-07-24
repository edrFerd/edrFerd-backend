use chrono;
use log::{error, info};
use serde_json::{json, Value};
use tokio::net::UdpSocket;

// 工作循环
pub async fn work_loop() -> anyhow::Result<()> {
    // 将套接字绑定到 "0.0.0.0:8080"，你可以根据需要更改端口
    let sock = UdpSocket::bind("0.0.0.0:8080").await?;
    info!("Listening on: {}", sock.local_addr()?);

    let mut buf = [0; 1024];

    loop {
        match sock.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                // 将接收到的字节转换为字符串
                let received_data = String::from_utf8_lossy(&buf[..len]);
                info!("从 {addr} 接收到数据: {received_data}");

                // 尝试将接收到的数据解析为 JSON
                let json_data = match serde_json::from_str::<Value>(&received_data) {
                    Ok(parsed_json) => {
                        info!("接收到有效的 JSON 数据: {parsed_json}");
                        parsed_json
                    }
                    Err(_) => {
                        // 如果不是有效的 JSON，创建一个包含原始数据的 JSON 对象
                        let wrapped_json = json!({
                            "raw_data": received_data.trim(),
                            "from_address": addr.to_string(),
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        });
                        info!("将原始数据包装为 JSON: {wrapped_json}");
                        wrapped_json
                    }
                };

                // 这里可以进一步处理 json_data
                // 例如存储到数据库、发送到其他服务等
                info!("处理完成的 JSON 数据: {json_data}");
            }
            Err(e) => {
                error!("接收到数据失败: {e}");
            }
        }
    }
}
