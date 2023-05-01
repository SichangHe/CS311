use std::{net::IpAddr, process::exit, str::FromStr};

use anyhow::{bail, Context, Result};
use log::debug;
use tokio::sync::mpsc::Receiver;

use crate::channel::Senders;

pub async fn handle(mut cmd_receiver: Receiver<String>, senders: Senders) {
    while let Some(buf) = cmd_receiver.recv().await {
        let input = buf.trim();
        // Ignore empty input.
        if input.is_empty() {
            senders
                .response
                .send("".into())
                .await
                .expect("Response receiver closed.");
            continue;
        }
        debug!("Received command `{}`.", buf.trim());
        match Command::parse(input) {
            Ok(cmd) => {
                debug!("Parsed command: `{cmd:?}`.");
                match cmd {
                    Command::Add { ip: _, weight: _ } => senders
                        .route
                        .send(cmd)
                        .await
                        .expect("Route receiver cclosed."),
                    Command::Del { ip: _ } => senders
                        .route
                        .send(cmd)
                        .await
                        .expect("Route receiver cclosed."),
                    Command::Trace { ip: _ } => todo!(),
                    Command::Quit => exit(0),
                }
                senders
                    .response
                    .send("".into())
                    .await
                    .expect("Response receiver closed.");
            }
            Err(err) => senders
                .response
                .send(err.to_string())
                .await
                .expect("Response receiver closed."),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Add { ip: IpAddr, weight: usize },
    Del { ip: IpAddr },
    Trace { ip: IpAddr },
    Quit,
}

impl Command {
    /// Parse trimmed, non-empty `input`.
    pub fn parse(input: &str) -> Result<Self> {
        let mut args = input.split_whitespace();
        match args.next().unwrap() {
            "add" => {
                let ip_str = args.next().context("Missing ip for add command.")?;
                let ip = IpAddr::from_str(ip_str)?;
                let weight = args
                    .next()
                    .context("Missing weight for add command.")?
                    .parse()?;
                Ok(Self::Add { ip, weight })
            }
            "del" => {
                let ip_str = args.next().context("Missing ip for del command.")?;
                let ip = IpAddr::from_str(ip_str)?;
                Ok(Self::Del { ip })
            }
            "trace" => {
                let ip_str = args.next().context("Missing ip for trace command.")?;
                let ip = IpAddr::from_str(ip_str)?;
                Ok(Self::Trace { ip })
            }
            "quit" => Ok(Self::Quit),
            other => {
                bail!("{other} is not a valid command.")
            }
        }
    }
}
