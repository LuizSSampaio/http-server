use std::collections::HashMap;

use http::StatusCode;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[derive(Debug, Default, Clone)]
pub struct Router {
    routes: radix_trie::Trie<String, HashMap<Method, fn()>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    pub fn route(&mut self, path: &str, method: Method, handler: fn()) {
        if !path.starts_with("/") {
            panic!("Route path must start with '/'");
        }

        match self.routes.get_mut(path) {
            Some(methods) => {
                methods.insert(method, handler);
            }
            None => {
                self.routes
                    .insert(path.to_string(), HashMap::from([(method, handler)]));
            }
        }
    }

    pub async fn serve(&self, addr: &str) {
        //FIX:
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind address");

        loop {
            let Ok((mut socket, _)) = listener.accept().await else {
                continue;
            };

            tokio::spawn(async move {
                let (reader, mut writer) = socket.split();
                let reader = BufReader::new(reader);
                let request = reader.lines().next_line().await.unwrap().unwrap();

                let status = match &request[..] {
                    "GET / HTTP/1.1" => StatusCode::OK,
                    _ => StatusCode::NOT_FOUND,
                };

                let response = format!(
                    "HTTP/1.1 {} {}\r\n\r\n",
                    status.as_str(),
                    status.canonical_reason().unwrap_or("")
                );

                writer.write_all(response.as_bytes()).await.unwrap();
            });
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    GET,
    HEAD,
    OPTIONS,
    TRACE,
    PUT,
    DELETE,
    POST,
    PATCH,
    CONNECT,
}
