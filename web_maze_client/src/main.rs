use std::{
    error::Error,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

type ResultDyn<T> = Result<T, Box<dyn Error>>;

fn main() -> ResultDyn<()> {
    let host = "127.0.0.1:4000";
    let mut stream = TcpStream::connect(host)?;
    let to_write = format!(
        "GET /api/list HTTP/1.1\r
Host: {host}\r
\r
"
    );
    println!("{to_write}");
    stream.write_all(to_write.as_bytes())?;
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
