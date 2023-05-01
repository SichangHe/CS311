pub mod command;

use std::io::stdin;

use command::handle;
use tokio::{spawn, sync::mpsc::channel};

#[tokio::main]
async fn main() {
    env_logger::init();
    let stdin = stdin();
    let (cmd_sender, cmd_receiver) = channel(0xFFF);
    let _command_handle = spawn(handle(cmd_receiver));
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).expect("Error reading Stdin.");
        cmd_sender
            .send(buf)
            .await
            .expect("Command handle is closed.");
    }
}
