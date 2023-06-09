use std::{net::IpAddr, sync::Arc};

use log::debug;
use tokio::{net::UdpSocket, spawn, sync::mpsc::Receiver};

use crate::{channel::Senders, message::Message};

const PORT: u16 = 55151;

pub async fn bind(addr: IpAddr, senders: Senders, mut send_receiver: Receiver<Send>) {
    let socket = UdpSocket::bind((addr, PORT))
        .await
        .expect("Failed to listen on port.");
    let socket = Arc::new(socket);
    let _listen = spawn(listen(socket.clone(), senders));
    while let Some(send) = send_receiver.recv().await {
        let msg = match serde_json::to_string(&send.msg) {
            Ok(m) => m,
            Err(err) => {
                debug!("{err}");
                continue;
            }
        };
        match socket.send_to(msg.as_bytes(), (send.to, PORT)).await {
            Ok(_) => debug!("Sent `{msg}` to {}", send.to),
            Err(err) => debug!("{err}"),
        }
    }
}

async fn listen(socket: Arc<UdpSocket>, senders: Senders) {
    let mut buf = vec![0u8; 0xFFF];
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((n_read, other_addr)) => {
                let msg = String::from_utf8_lossy(&buf[..n_read]);
                debug!("Received `{msg}` from `{other_addr}`.");
                senders.msg(msg.into()).await;
                init_zero(&mut buf, n_read);
            }
            Err(err) => println!("{err}"),
        }
    }
}

fn init_zero(vec: &mut [u8], length: usize) {
    (0..length).for_each(|index| {
        vec[index] = 0;
    });
}

#[derive(Debug)]
pub struct Send {
    pub to: IpAddr,
    pub msg: Message,
}
