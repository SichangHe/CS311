use std::{net::IpAddr, str::FromStr};

use anyhow::{bail, Context, Result};
use log::debug;
use tokio::sync::mpsc::Receiver;

pub async fn handle(mut cmd_receiver: Receiver<String>) {
    while let Some(buf) = cmd_receiver.recv().await {
        let input = buf.trim();
        // Ignore empty input.
        if input.is_empty() {
            continue;
        }
        debug!("Received command `{}`.", buf.trim());
        match Command::parse(input) {
            Ok(cmd) => {
                debug!("Parsed command: `{cmd:?}`.")
            }
            Err(err) => {
                println!("{err}")
            }
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Add { ip: IpAddr, weight: usize },
    Del { ip: IpAddr },
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
            other => {
                bail!("{other} is not a valid command.")
            }
        }
    }
}
