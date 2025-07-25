use crate::libs::data_struct::Chunk;

const WORK_INTERVAL: f64 = 1.0 / 20.0;
async fn work_loop() {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs_f64(WORK_INTERVAL)).await;
        for (){

        }
    }
}

async fn handle_chunk(chunk: Chunk) {

}
