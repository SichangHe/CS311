use std::{net::IpAddr, process::exit};

use clap::Parser;
use log::debug;
use tokio::sync::mpsc::Receiver;

use crate::{channel::Senders, route::Route};

pub async fn handle(mut cmd_receiver: Receiver<String>, senders: Senders) {
    while let Some(buf) = cmd_receiver.recv().await {
        let input = buf.trim();
        // Ignore empty input.
        if input.is_empty() {
            senders.response("".into()).await;
            continue;
        }
        debug!("Received command `{input}`.");
        // Provide first command to Clap.
        let mut clap_input = vec![""];
        clap_input.extend(input.split_whitespace());
        match Command::try_parse_from(clap_input) {
            Ok(cmd) => {
                debug!("Parsed command: `{cmd:?}`.");
                match cmd {
                    Command::Add { ip, weight } => senders.route(Route::Add { ip, weight }).await,
                    Command::Del { ip } => senders.route(Route::Del { ip }).await,
                    Command::Trace { ip: _ } => todo!(),
                    Command::Quit => exit(0),
                }
                senders.response("".into()).await;
            }
            Err(err) => senders.response(err.to_string()).await,
        }
    }
}

#[derive(Debug, Parser)]
pub enum Command {
    Add {
        #[arg(index = 1)]
        ip: IpAddr,
        #[arg(index = 2)]
        weight: f64,
    },
    Del {
        #[arg(index = 1)]
        ip: IpAddr,
    },
    Trace {
        #[arg(index = 1)]
        ip: IpAddr,
    },
    Quit,
}
