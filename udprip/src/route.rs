use log::debug;
use rand::seq::SliceRandom;
use std::{collections::BTreeMap, net::IpAddr};
use tokio::sync::mpsc::Receiver;

use crate::{channel::Senders, message::Message, socket::Send};

pub async fn manage(senders: Senders, mut cmd_receiver: Receiver<Route>) {
    let mut accept = BTreeMap::new();
    let mut peer: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();
    let mut next = BTreeMap::new();
    while let Some(route) = cmd_receiver.recv().await {
        debug!("Route: {route:#?}");
        match route {
            Route::Add { ip, weight } => {
                accept.insert(ip, weight);
                debug!("`accept` is now {accept:#?}");
                // TODO: Notify peers.
            }
            Route::Del { ip } => {
                accept.remove(&ip);
                debug!("`accept` is now {accept:#?}");
                // TODO: Notify peers.
            }
            Route::Update { source, distances } => {
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
            Route::Forward { msg } => {
                let to = match next.get(&msg.destination) {
                    Some((_, paths)) => *paths
                        .choose(&mut rand::thread_rng())
                        .expect("Paths vector is empty"),
                    None => {
                        // Drop the package.
                        debug!("No next hop found for {msg:?}.");
                        continue;
                    }
                };
                senders.send(Send { to, msg }).await;
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
pub enum Route {
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
    Forward {
        msg: Message,
    },
}
