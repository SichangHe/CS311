use tokio::sync::mpsc::Sender;

use crate::route::Update;

#[derive(Clone)]
pub struct Senders {
    pub cmd: Sender<String>,
    pub response: Sender<String>,
    pub route: Sender<Update>,
    pub msg: Sender<String>,
}

impl Senders {
    pub async fn cmd(&self, msg: String) {
        self.cmd.send(msg).await.expect("Command receiver closed.")
    }
    pub async fn response(&self, msg: String) {
        self.response
            .send(msg)
            .await
            .expect("Response receiver closed.")
    }
    pub async fn route(&self, msg: Update) {
        self.route.send(msg).await.expect("Route receiver closed.")
    }
    pub async fn msg(&self, msg: String) {
        self.msg.send(msg).await.expect("Msg receiver closed.")
    }
}
