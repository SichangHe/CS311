use anyhow::{bail, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, net::IpAddr};
use tokio::sync::mpsc::Receiver;

use crate::{channel::Senders, route::Route};

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

pub async fn act(addr: IpAddr, mut msg: Message, senders: &Senders) -> Result<()> {
    match msg.tipe.as_str() {
        "data" => {
            let payload = match &msg.payload {
                Some(p) => p,
                None => bail!("Data message without payload `{msg:?}`."),
            };
            if msg.destination == addr {
                // This message is for me.
                println!("{payload}");
            } else {
                senders.route(Route::Forward { msg }).await;
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
                .route(Route::Update {
                    source: msg.source,
                    distances,
                })
                .await
        }
        "trace" => {
            let routers = match &mut msg.routers {
                Some(r) => r,
                None => bail!("Trace message without routers: `{msg:?}`."),
            };
            routers.push(addr);
            if msg.destination == addr {
                let msg = Message {
                    source: addr,
                    destination: msg.source,
                    tipe: "data".into(),
                    payload: Some(
                        serde_json::to_string(&msg).expect("Failed to parse message as JSON."),
                    ),
                    distances: None,
                    routers: None,
                };
                senders.route(Route::Forward { msg }).await;
            } else {
                senders.route(Route::Forward { msg }).await;
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
    pub distances: Option<BTreeMap<IpAddr, usize>>,
    pub routers: Option<Vec<IpAddr>>,
}
