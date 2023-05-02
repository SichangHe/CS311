use anyhow::{bail, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, net::IpAddr};
use tokio::sync::mpsc::Receiver;

use crate::{channel::Senders, route::Update};

pub async fn listen(addr: IpAddr, mut msg_receiver: Receiver<String>, senders: Senders) {
    while let Some(msg) = msg_receiver.recv().await {
        match serde_json::from_str::<Message>(&msg) {
            Ok(message) => {
                debug!("{message:#?}");
                if let Err(err) = act(addr, message, &senders).await {
                    debug!("{err}");
                }
            }
            Err(err) => debug!("{err}: `{msg}`"),
        }
    }
}

pub async fn act(addr: IpAddr, msg: Message, senders: &Senders) -> Result<()> {
    match msg.tipe.as_str() {
        "data" => {
            let payload = match msg.payload {
                Some(p) => p,
                None => bail!("Data message without payload `{msg:?}`."),
            };
            if msg.destination == addr {
                // This message is for me.
                println!("{payload}");
            } else {
                // TODO: Forward the message.
            }
        }
        "update" => {
            if msg.destination != addr {
                bail!("Update message not for a neighbor: {msg:?}.")
            }
            let distances = match msg.distances {
                Some(d) => d,
                None => bail!("Update message without distances: `{msg:?}."),
            };
            senders
                .route(Update::Update {
                    source: msg.source,
                    distances,
                })
                .await
        }
        "trace" => {
            let mut routers = match msg.routers {
                Some(r) => r,
                None => bail!("Trace message without routers: `{msg:?}`."),
            };
            routers.push(addr);
            if msg.destination == addr {
                // TODO: Respond.
            } else {
                // TODO: Forward.
            }
        }
        _ => debug!("Unknow message type: `{msg:?}`."),
    }
    Ok(())
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
