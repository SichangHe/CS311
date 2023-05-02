use std::{collections::BTreeMap, net::IpAddr};

use log::debug;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Receiver;

use crate::channel::Senders;

pub async fn listen(mut msg_receiver: Receiver<String>, senders: Senders) {
    while let Some(msg) = msg_receiver.recv().await {
        match serde_json::from_str::<Message>(&msg) {
            Ok(message) => {
                debug!("{message:#?}");
                // TODO: Handle message.
            }
            Err(err) => debug!("{err}: `{msg}`"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub source: IpAddr,
    pub destination: IpAddr,
    #[serde(rename = "type")]
    pub tipe: String,
    pub payload: Option<String>,
    pub distances: Option<BTreeMap<IpAddr, f64>>,
    pub routers: Option<Vec<IpAddr>>,
}
