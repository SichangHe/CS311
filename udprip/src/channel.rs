use tokio::sync::mpsc::Sender;

use crate::{route::Route, socket::Send};

#[derive(Clone)]
pub struct Senders {
    pub cmd: Sender<String>,
    pub response: Sender<String>,
    pub route: Sender<Route>,
    pub msg: Sender<String>,
    pub send: Sender<Send>,
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
    pub async fn route(&self, msg: Route) {
        self.route.send(msg).await.expect("Route receiver closed.")
    }
    pub async fn msg(&self, msg: String) {
        self.msg.send(msg).await.expect("Msg receiver closed.")
    }
    pub async fn send(&self, msg: Send) {
        self.send.send(msg).await.expect("Send receiver closed.")
    }
}
