use std::{collections::BTreeMap, net::IpAddr};

use log::debug;
use tokio::sync::mpsc::Receiver;

pub async fn manage(mut cmd_receiver: Receiver<Update>) {
    let mut accept = BTreeMap::new();
    let mut peer: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();
    let mut next = BTreeMap::new();
    while let Some(update) = cmd_receiver.recv().await {
        debug!("Update: {update:#?}");
        match update {
            Update::Add { ip, weight } => {
                accept.insert(ip, weight);
                debug!("`accept` is now {accept:#?}");
                // TODO: Notify peers.
            }
            Update::Del { ip } => {
                accept.remove(&ip);
                debug!("`accept` is now {accept:#?}");
                // TODO: Notify peers.
            }
            Update::Update { source, distances } => {
                for (addr, distance) in distances {
                    match peer.get_mut(&addr) {
                        Some(paths) => {
                            paths.insert(source, distance);
                        }
                        None => {
                            let mut paths = BTreeMap::new();
                            paths.insert(source, distance);
                            peer.insert(addr, paths);
                        }
                    }
                }
                debug!("`peer` is now {peer:#?}.");
                next = calculate_next(&peer);
                debug!("`next` is now {next:#?}.");
            }
        }
    }
}

fn calculate_next(
    peer: &BTreeMap<IpAddr, BTreeMap<IpAddr, f64>>,
) -> BTreeMap<IpAddr, (f64, Vec<IpAddr>)> {
    let mut next = BTreeMap::new();
    for (&destination, paths) in peer {
        let mut shortest_distance = f64::MAX;
        let mut shortest_paths = Vec::with_capacity(1);
        for (&path, &distance) in paths {
            if distance < shortest_distance {
                shortest_distance = distance;
                shortest_paths.clear();
                shortest_paths.push(path);
            } else if distance == shortest_distance {
                shortest_paths.push(path)
            }
        }
        next.insert(destination, (shortest_distance, shortest_paths));
    }
    next
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
