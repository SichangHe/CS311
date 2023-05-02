use log::debug;
use rand::seq::SliceRandom;
use std::{cmp::Ordering, collections::BTreeMap, net::IpAddr};
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
                peer.insert(source, distances);
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
    accept: &BTreeMap<IpAddr, usize>,
    next: &BTreeMap<IpAddr, (usize, Vec<IpAddr>)>,
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
    accept: &BTreeMap<IpAddr, usize>,
    peer: &BTreeMap<IpAddr, BTreeMap<IpAddr, usize>>,
) -> BTreeMap<IpAddr, (usize, Vec<IpAddr>)> {
    let mut next = BTreeMap::new();
    for (&destination, &weight) in accept {
        next.insert(destination, (weight, vec![destination]));
    }
    for (&source, paths) in peer {
        for (&destination, &distance) in paths {
            let weight = match accept.get(&source) {
                Some(&w) => w,
                None => {
                    // Ignore sources that are not accepted.
                    continue;
                }
            } + distance;

            next.entry(destination)
                .and_modify(|(old_weight, sources)| match weight.cmp(old_weight) {
                    Ordering::Less => {
                        *old_weight = weight;
                        sources.clear();
                        sources.push(source);
                    }
                    Ordering::Equal => sources.push(source),
                    _ => (),
                })
                .or_insert((weight, vec![source]));
        }
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
        distances: BTreeMap<IpAddr, usize>,
    },
    Forward {
        msg: Message,
    },
}
