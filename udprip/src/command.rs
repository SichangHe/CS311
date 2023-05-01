use tokio::sync::mpsc::Receiver;

pub async fn handle(mut cmd_receiver: Receiver<String>) {
    while let Some(buf) = cmd_receiver.recv().await {
        println!("{buf}")
    }
}
