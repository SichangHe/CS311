use std::{collections::BTreeMap, net::IpAddr};

use log::debug;
use tokio::sync::mpsc::Receiver;

pub async fn manage(mut cmd_receiver: Receiver<Update>) {
    let mut accept = BTreeMap::new();
    while let Some(update) = cmd_receiver.recv().await {
        debug!("Update: {update:#?}");
        match update {
            Update::Add { ip, weight } => {
                accept.insert(ip, weight);
            }
            Update::Del { ip } => {
                accept.remove(&ip);
            }
            Update::Update { source: _, distances: _ } => {
                // TODO: Handle update message.
            }
        }
        // TODO: Notify peers.
        debug!("`accept` is now {accept:#?}");
    }
}

#[derive(Debug)]
pub enum Update {
    Add {
        ip: IpAddr,
        weight: usize,
    },
    Del {
        ip: IpAddr,
    },
    Update {
        source: IpAddr,
        distances: BTreeMap<IpAddr, f64>,
    },
}
