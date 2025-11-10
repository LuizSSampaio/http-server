use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use http::StatusCode;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_client(stream) {
                    println!("error: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) -> anyhow::Result<()> {
    let reader = BufReader::new(&stream);
    let request = reader.lines().next().unwrap().unwrap();

    let status = match &request[..] {
        "GET / HTTP/1.1" => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    };

    let response = format!(
        "HTTP/1.1 {} {}\r\n\r\n",
        status.as_str(),
        status.canonical_reason().unwrap_or("")
    );
    match stream.write_all(response.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => anyhow::bail!(e),
    }
}
