use tokio::sync::mpsc::Sender;

use crate::command::Command;

#[derive(Clone)]
pub struct Senders {
    pub cmd: Sender<String>,
    pub response: Sender<String>,
    pub route: Sender<Command>,
    pub msg: Sender<String>,
}
