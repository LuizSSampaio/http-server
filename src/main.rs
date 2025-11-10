use std::{
    io::Write,
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
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    match stream.write_all(response.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => anyhow::bail!(e),
    }
}
