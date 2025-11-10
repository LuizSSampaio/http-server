use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

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
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK",
        _ => "HTTP/1.1 404 Not Found",
    };

    let response = format!("{status}\r\n\r\n");
    match stream.write_all(response.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => anyhow::bail!(e),
    }
}
