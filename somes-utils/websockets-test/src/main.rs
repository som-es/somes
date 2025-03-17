use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;

const WS_URL: &str = "ws://localhost:3000/quiz_room"; 
const NUM_CONNECTIONS: usize = 800;

async fn connect_and_send(index: usize) {
    if let Ok((mut ws_stream, _)) = connect_async(WS_URL).await {
        if ws_stream.send("b".into()).await.is_ok() {
            if let Some(Ok(msg)) = ws_stream.next().await {
                println!("Connection {}: {:?}", index, msg);
            }
        }
        sleep(Duration::from_secs(10)).await; 
    } else {
        eprintln!("Connection {} failed", index);
    }
}

#[tokio::main]
async fn main() {
    let mut tasks = Vec::new();
    for i in 0..NUM_CONNECTIONS {
        tasks.push(task::spawn(connect_and_send(i)));
    }
    
    for task in tasks {
        let _ = task.await;
    }
}
