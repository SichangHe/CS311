pub mod channel;
pub mod command;
pub mod message;
pub mod route;
pub mod socket;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    net::IpAddr,
    path::{Path, PathBuf},
    process::exit,
};

use clap::Parser;
use command::handle;
use log::debug;
use rustyline::{error::ReadlineError, history::FileHistory, Editor};
use tokio::{
    spawn,
    sync::mpsc::{channel, Receiver},
};

use crate::{channel::Senders, message::listen, route::manage, socket::bind};

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    debug!("{args:?}");
    let (cmd, cmd_receiver) = channel(1);
    let (response, mut response_receiver) = channel(2);
    let (route, route_receiver) = channel(8);
    let (msg, msg_receiver) = channel(64);
    let (send, send_receiver) = channel(8);
    let senders = Senders {
        cmd,
        response,
        route,
        msg,
        send,
    };
    let _socket = spawn(bind(args.address, senders.clone(), send_receiver));
    let _command_handle = spawn(handle(args.address, cmd_receiver, senders.clone()));
    let _route_manager = spawn(manage(args.address, senders.clone(), route_receiver));
    let _msg_listener = spawn(listen(args.address, msg_receiver, senders.clone()));
    if let Some(path) = args.startup {
        startup_file(&path, &senders, &mut response_receiver).await;
    }
    let mut editor = rustyline::DefaultEditor::new().expect("Failed to initialize RustyLine");
    loop {
        read_line(&mut editor, &senders, &mut response_receiver).await;
    }
}

/// Read one line from RustyLine editor.
async fn read_line(
    editor: &mut Editor<(), FileHistory>,
    senders: &Senders,
    response_receiver: &mut Receiver<String>,
) {
    match editor.readline("> ") {
        Ok(line) => {
            if let Err(err) = editor.add_history_entry(&line) {
                println!("{err}");
            }
            handle_line(line, senders, response_receiver).await;
        }
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => exit(0),
        Err(err) => println!("{err}"),
    }
}

/// Read `startup` file.
async fn startup_file(path: &Path, senders: &Senders, response_receiver: &mut Receiver<String>) {
    let file = File::open(path).expect("Failed to open startup file.");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Error reading lines in startup file.");
        handle_line(line, senders, response_receiver).await;
    }
}

/// Handle one line read.
async fn handle_line(line: String, senders: &Senders, response_receiver: &mut Receiver<String>) {
    senders.cmd(line).await;
    let response = response_receiver
        .recv()
        .await
        .expect("Response sender closed.");
    if !response.is_empty() {
        println!("{response}")
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
