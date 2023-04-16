use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use log::debug;

use crate::{response, ResultDyn};

pub fn submit(host: &str, id: &str) -> ResultDyn<response::Submit> {
    let body = request(host, &format!("/api/run/{id}"), "POST")?;
    Ok(serde_json::from_str(&body)?)
}

pub fn list(host: &str, limit: Option<usize>, start: Option<usize>) -> ResultDyn<response::List> {
    let path = "/api/list".to_owned();
    let path = match (limit, start) {
        (None, None) => path,
        (Some(limit), None) => format!("{path}?limit={limit}"),
        (None, Some(start)) => format!("{path}?start={start}"),
        (Some(limit), Some(start)) => format!("{path}?limit={limit}&start={start}"),
    };
    let body = request(host, &path, "GET")?;

    Ok(serde_json::from_str(&body)?)
}

pub fn queries(
    host: &str,
    run_id: &str,
    limit: Option<usize>,
    start: Option<usize>,
) -> ResultDyn<response::Queries> {
    let path = format!("/api/queries?run={run_id}");
    let path = match (limit, start) {
        (None, None) => path,
        (Some(limit), None) => format!("{path}&limit={limit}"),
        (None, Some(start)) => format!("{path}&start={start}"),
        (Some(limit), Some(start)) => format!("{path}&limit={limit}&start={start}"),
    };
    let body = request(host, &path, "GET")?;

    Ok(serde_json::from_str(&body)?)
}

pub fn request(host: &str, path: &str, method: &str) -> ResultDyn<String> {
    let mut stream = TcpStream::connect(host)?;
    let to_write = format!(
        "{method} {path} HTTP/1.1\r
Host: {host}\r
\r
"
    );
    debug!("{to_write}");
    stream.write_all(to_write.as_bytes())?;

    let mut content_length: Option<usize> = None;
    let mut reader = BufReader::new(stream);

    // Read header.
    let mut headers = String::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        if line.to_lowercase().starts_with("content-length:") {
            content_length = Some(line.split_once(": ").unwrap().1.trim().parse()?)
        }
        headers += &line;
    }
    debug!("{headers}");

    // Read body.
    let size = content_length.expect("No `content-length' in header");
    let mut buf = vec![0u8; size];
    reader.read_exact(&mut buf)?;
    let body = String::from_utf8_lossy(&buf).into();
    debug!("{body}");

    Ok(body)
}
