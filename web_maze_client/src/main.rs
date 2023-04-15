use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use clap::{arg, command, Parser};
use web_maze_client::{response, ResultDyn};

fn main() -> ResultDyn<()> {
    let args = Args::parse();
    eprintln!("{args:#?}");
    match args.cmd {
        Act::Submit { id } => {
            submit(&args.url, &id)?;
        }
        Act::List { limit, start } => {
            list(&args.url, limit, start)?;
        }
        Act::Stats => todo!(),
    }
    Ok(())
}

fn submit(host: &str, id: &str) -> ResultDyn<response::Submit> {
    let body = request(host, &format!("/api/run/{id}"), "POST")?;
    let submit_response = serde_json::from_str(&body)?;

    println!("\n{submit_response:#?}");
    Ok(submit_response)
}

fn list(host: &str, limit: Option<usize>, start: Option<usize>) -> ResultDyn<response::List> {
    let path = "/api/list".to_owned();
    let path = match (limit, start) {
        (None, None) => path,
        (Some(limit), None) => format!("{path}?limit={limit}"),
        (None, Some(start)) => format!("{path}?start={start}"),
        (Some(limit), Some(start)) => format!("{path}?limit={limit}&start={start}"),
    };
    let body = request(host, &path, "GET")?;

    let list_response = serde_json::from_str(&body)?;
    println!("\n{list_response:#?}");
    Ok(list_response)
}

fn request(host: &str, path: &str, method: &str) -> ResultDyn<String> {
    let mut stream = TcpStream::connect(host)?;
    let to_write = format!(
        "{method} {path} HTTP/1.1\r
Host: {host}\r
\r
"
    );
    println!("{to_write}");
    stream.write_all(to_write.as_bytes())?;

    let mut content_length: Option<usize> = None;
    let mut reader = BufReader::new(stream);

    // Read header.
    let mut line = String::new();
    loop {
        reader.read_line(&mut line)?;
        print!("{line}");
        if line.trim().is_empty() {
            break;
        }
        if line.to_lowercase().starts_with("content-length:") {
            content_length = Some(line.split_once(": ").unwrap().1.trim().parse()?)
        }
        line.clear();
    }

    // Read body.
    let size = content_length.expect("No `content-length' in header");
    let mut buf = vec![0u8; size];
    reader.read_exact(&mut buf)?;
    let body = String::from_utf8_lossy(&buf).into();
    println!("{body}");

    Ok(body)
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[command(subcommand)]
    cmd: Act,

    #[arg(short, long)]
    url: String,
}

#[derive(Parser, Debug)]
enum Act {
    Submit {
        #[arg(short, long)]
        id: String,
    },
    List {
        #[arg(short, long)]
        limit: Option<usize>,

        #[arg(short, long)]
        start: Option<usize>,
    },
    Stats,
}
