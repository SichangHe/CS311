pub mod command;

use command::handle;
use rustyline::error::ReadlineError;
use tokio::{spawn, sync::mpsc::channel};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut editor = rustyline::DefaultEditor::new().expect("Failed to initialize RustyLine");
    let (cmd_sender, cmd_receiver) = channel(0xFFF);
    let (response_sender, mut response_receiver) = channel(2);
    let _command_handle = spawn(handle(cmd_receiver, response_sender));
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
                println!("{response}")
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => println!("{err}"),
        }
    }
}
