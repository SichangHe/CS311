use log::debug;
use rand::seq::SliceRandom;
use std::{collections::BTreeMap, net::IpAddr};
use tokio::sync::mpsc::Receiver;

use crate::{channel::Senders, message::Message, socket::Send};

pub async fn manage(addr: IpAddr, senders: Senders, mut cmd_receiver: Receiver<Route>) {
    let mut accept = BTreeMap::new();
    let mut peer: BTreeMap<_, BTreeMap<_, _>> = BTreeMap::new();
    let mut next = BTreeMap::new();
    while let Some(route) = cmd_receiver.recv().await {
        debug!("Route: {route:#?}");
        match route {
            Route::Add { ip, weight } => {
                accept.insert(ip, weight);
                debug!("`accept` is now {accept:#?}");
                next = calculate_next(&accept, &peer);
                notify(addr, &accept, &next, &senders).await;
            }
            Route::Del { ip } => {
                accept.remove(&ip);
                debug!("`accept` is now {accept:#?}");
                next = calculate_next(&accept, &peer);
                notify(addr, &accept, &next, &senders).await;
            }
            Route::Update { source, distances } => {
                for (destination, distance) in distances {
                    peer.insert_path(destination, source, distance);
                }
                next = calculate_next(&accept, &peer);
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

async fn notify(
    source: IpAddr,
    accept: &BTreeMap<IpAddr, f64>,
    next: &BTreeMap<IpAddr, (f64, Vec<IpAddr>)>,
    senders: &Senders,
) {
    for &to in accept.keys() {
        let mut distances = BTreeMap::new();
        for (&destination, (distance, paths)) in next {
            if destination == to || paths.contains(&to) {
                // Skip routes that goes to `to`.
                continue;
            }
            distances.insert(destination, *distance);
        }
        let msg = Message {
            source,
            destination: to,
            tipe: "update".into(),
            payload: None,
            distances: Some(distances.clone()),
            routers: None,
        };
        senders.send(Send { to, msg }).await;
    }
}

fn calculate_next(
    accept: &BTreeMap<IpAddr, f64>,
    peer: &BTreeMap<IpAddr, BTreeMap<IpAddr, f64>>,
) -> BTreeMap<IpAddr, (f64, Vec<IpAddr>)> {
    let mut next = BTreeMap::new();
    for (&destination, &weight) in accept {
        next.insert(destination, (weight, vec![destination]));
    }
    for (&destination, paths) in peer {
        let (mut min_weight, mut best_sources) = next
            .remove(&destination)
            .unwrap_or((f64::MAX, Vec::with_capacity(1)));
        for (&source, &distance) in paths {
            let weight = match accept.get(&source) {
                Some(&w) => w,
                None => {
                    // Ignore sources that are not accepted.
                    continue;
                }
            } + distance;
            if weight < min_weight {
                min_weight = weight;
                best_sources.clear();
                best_sources.push(source);
            } else if weight == min_weight {
                best_sources.push(source)
            }
        }
        next.insert(destination, (min_weight, best_sources));
    }
    next
}

trait Peers {
    fn insert_path(&mut self, destination: IpAddr, source: IpAddr, distance: f64);
    fn remove_source(&mut self, source: &IpAddr);
}

impl Peers for BTreeMap<IpAddr, BTreeMap<IpAddr, f64>> {
    fn insert_path(&mut self, destination: IpAddr, source: IpAddr, distance: f64) {
        match self.get_mut(&destination) {
            Some(paths) => {
                paths.insert(source, distance);
            }
            None => {
                let mut paths = BTreeMap::new();
                paths.insert(source, distance);
                self.insert(destination, paths);
            }
        }
    }

    fn remove_source(&mut self, source: &IpAddr) {
        for paths in self.values_mut() {
            paths.remove(source);
        }
    }
}

#[derive(Debug)]
pub enum Route {
    Add {
        ip: IpAddr,
        weight: f64,
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
