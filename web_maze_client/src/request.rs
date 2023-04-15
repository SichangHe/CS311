use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::ResultDyn;

pub fn request(host: &str, path: &str, method: &str) -> ResultDyn<String> {
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
