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

use crate::route::manage;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    debug!("{args:?}");
    let (cmd_sender, cmd_receiver) = channel(1);
    let (response_sender, mut response_receiver) = channel(2);
    let (route_sender, route_receiver) = channel(8);
    let _command_handle = spawn(handle(cmd_receiver, response_sender, route_sender));
    let _route_manager = spawn(manage(route_receiver));
    // Read `startup` file.
    if let Some(path) = args.startup {
        let file = File::open(path).expect("Failed to open startup file.");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.expect("Error reading lines in startup file.");
            cmd_sender
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
                cmd_sender
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
