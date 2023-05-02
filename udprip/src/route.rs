use log::debug;
use rand::seq::SliceRandom;
use std::{cmp::Ordering, collections::BTreeMap, net::IpAddr, time::Duration};
use tokio::{
    spawn,
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
    time::timeout,
};

use crate::{channel::Senders, message::Message, socket::Send};

pub async fn manage(
    addr: IpAddr,
    period: Duration,
    senders: Senders,
    mut cmd_receiver: Receiver<Route>,
) {
    let time_out = period * 4;
    let mut accept = BTreeMap::new();
    let mut peers = Peers::new();
    let mut next = BTreeMap::new();
    let (notify_sender, notify_receiver) = channel(2);
    let _time_notify = spawn(time_notify(period, senders.clone(), notify_receiver));
    while let Some(route) = cmd_receiver.recv().await {
        debug!("Route: {route:?}");
        match route {
            Route::Add { ip, weight } => {
                accept.insert(ip, weight);
                debug!("`accept` is now {accept:?}");
                next = calculate_next(&accept, &peers.peers);
                notify(addr, &accept, &next, &senders, &notify_sender).await;
            }
            Route::Del { ip } => {
                accept.remove(&ip);
                debug!("`accept` is now {accept:?}");
                next = calculate_next(&accept, &peers.peers);
                notify(addr, &accept, &next, &senders, &notify_sender).await;
            }
            Route::Update { source, distances } => {
                peers
                    .insert(source, distances, time_out, senders.clone())
                    .await;
                next = calculate_next(&accept, &peers.peers);
            }
            Route::Forward { msg } => send(&next, &msg.destination.clone(), msg, &senders).await,
            Route::Notify => notify(addr, &accept, &next, &senders, &notify_sender).await,
            Route::DelPeer { ip } => {
                debug!("Remove dead peer {ip}.");
                peers.remove(&ip).await;
                next = calculate_next(&accept, &peers.peers);
                debug!("`next` is now {next:?}.");
            }
        }
    }
}

async fn timeout_peer(
    ip: IpAddr,
    time_out: Duration,
    senders: Senders,
    mut peer_receiver: Receiver<()>,
) {
    while (timeout(time_out, peer_receiver.recv()).await).is_ok() {}
    senders.route(Route::DelPeer { ip }).await;
}

async fn time_notify(period: Duration, senders: Senders, mut notify_receiver: Receiver<()>) {
    loop {
        if (timeout(period, notify_receiver.recv()).await).is_err() {
            senders.route(Route::Notify).await;
        }
    }
}

async fn send(
    next: &BTreeMap<IpAddr, (usize, Vec<IpAddr>)>,
    destination: &IpAddr,
    msg: Message,
    senders: &Senders,
) {
    match next.get(destination) {
        Some((_, paths)) => {
            let to = *paths
                .choose(&mut rand::thread_rng())
                .expect("Paths vector is empty");
            senders.send(Send { to, msg }).await;
        }
        None => {
            // Drop the package.
            debug!("No next hop found for {msg:?}.");
        }
    };
}

async fn notify(
    source: IpAddr,
    accept: &BTreeMap<IpAddr, usize>,
    next: &BTreeMap<IpAddr, (usize, Vec<IpAddr>)>,
    senders: &Senders,
    notify_sender: &Sender<()>,
) {
    // Reset notify timer.
    notify_sender
        .send(())
        .await
        .expect("Notify receiver closed.");
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

struct Peers {
    peers: BTreeMap<IpAddr, BTreeMap<IpAddr, usize>>,
    timeout_peers: BTreeMap<IpAddr, JoinHandle<()>>,
    peer_senders: BTreeMap<IpAddr, Sender<()>>,
}

impl Peers {
    fn new() -> Self {
        Self {
            peers: BTreeMap::new(),
            timeout_peers: BTreeMap::new(),
            peer_senders: BTreeMap::new(),
        }
    }

    async fn insert(
        &mut self,
        ip: IpAddr,
        distances: BTreeMap<IpAddr, usize>,
        time_out: Duration,
        senders: Senders,
    ) {
        self.peers.insert(ip, distances);
        match self.peer_senders.get(&ip) {
            // Reset timeout_peer timeout.
            Some(peer_sender) => peer_sender.send(()).await.expect("Peer receiver closed."),
            None => {
                let (peer_sender, peer_receiver) = channel(1);
                let process = spawn(timeout_peer(ip, time_out, senders, peer_receiver));
                self.timeout_peers.insert(ip, process);
                self.peer_senders.insert(ip, peer_sender);
            }
        }
    }

    async fn remove(&mut self, ip: &IpAddr) {
        self.timeout_peers.remove(ip);
        self.peer_senders.remove(ip);
        self.peers.remove(ip);
    }
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
    Notify,
    DelPeer {
        ip: IpAddr,
    },
}
