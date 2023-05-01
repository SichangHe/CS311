pub mod channel;
pub mod command;
pub mod route;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    net::IpAddr,
    path::PathBuf,
};

use clap::Parser;
use command::handle;
use log::debug;
use rustyline::error::ReadlineError;
use tokio::{spawn, sync::mpsc::channel};

use crate::{channel::Senders, route::manage};

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    debug!("{args:?}");
    let (cmd, cmd_receiver) = channel(1);
    let (response, mut response_receiver) = channel(2);
    let (route, route_receiver) = channel(8);
    let senders = Senders {
        cmd,
        response,
        route,
    };
    let _command_handle = spawn(handle(cmd_receiver, senders.clone()));
    let _route_manager = spawn(manage(route_receiver));
    // Read `startup` file.
    if let Some(path) = args.startup {
        let file = File::open(path).expect("Failed to open startup file.");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Error reading lines in startup file.");
            senders
                .cmd
                .send(line)
                .await
                .expect("Command handle is closed.");
            let response = response_receiver
                .recv()
                .await
                .expect("Response sender closed.");
            if !response.is_empty() {
                println!("{response}")
            }
        }
    }
    let mut editor = rustyline::DefaultEditor::new().expect("Failed to initialize RustyLine");
    loop {
        match editor.readline("> ") {
            Ok(buf) => {
                if let Err(err) = editor.add_history_entry(&buf) {
                    println!("{err}");
                }
                senders
                    .cmd
                    .send(buf)
                    .await
                    .expect("Command handle is closed.");
                let response = response_receiver
                    .recv()
                    .await
                    .expect("Response sender closed.");
                if !response.is_empty() {
                    println!("{response}")
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => println!("{err}"),
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1)]
    address: IpAddr,
    #[arg(index = 2)]
    period: f64,
    #[arg(index = 3)]
    startup: Option<PathBuf>,
}
