use std::collections::BTreeMap;

use log::debug;
use tokio::sync::mpsc::Receiver;

use crate::command::Command;

pub async fn manage(mut cmd_receiver: Receiver<Command>) {
    let mut accept = BTreeMap::new();
    loop {
        match cmd_receiver.recv().await.unwrap() {
            Command::Add { ip, weight } => {
                accept.insert(ip, weight);
            }
            Command::Del { ip } => {
                accept.remove(&ip);
            }
            _ => unreachable!("Route managers should only receive Add and Del commands."),
        }
        // TODO: Notify peers.
        debug!("`accept` is now {accept:#?}");
    }
}
