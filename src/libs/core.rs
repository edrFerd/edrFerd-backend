use log::{error, info};
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
                info!("{} bytes received from {}", len, addr);
                // 在这里处理接收到的数据: &buf[..len]
            }
            Err(e) => {
                error!("Failed to receive datagram: {}", e);
            }
        }
    }
}
