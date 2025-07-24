const WORK_LOOP_INTERVAL: f64 = 1.0 / 20.0;
// 工作循环
async fn work_loop() {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(WORK_LOOP_INTERVAL)).await;
    }
}
