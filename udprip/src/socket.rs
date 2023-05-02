use std::net::IpAddr;

use log::debug;
use tokio::net::UdpSocket;

use crate::channel::Senders;

const PORT: u16 = 55151;

pub async fn bind(addr: IpAddr, senders: Senders) {
    let socket = UdpSocket::bind((addr, PORT))
        .await
        .expect("Failed to listen on port.");
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
