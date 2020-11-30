use tokio;
use std::net::{SocketAddr, TcpStream};
use std::io::prelude::*;
use std::error::Error;

pub struct Client {
    target: String,
    port: u16,
    stream: Option<TcpStream>
}

impl Client {
    pub fn new(target: String, port: u16) -> Client {
        return Client {
            target: target,
            port: port,
            stream: None
        }
    }

    pub fn connect(&mut self) {
        match TcpStream::connect(format!("{}:{}", self.target, self.port)) {
            Ok(stream) => self.stream = Some(stream),
            Err(e) => eprintln!("Error connecting to server: {}", e),
        }
    }

    pub fn send(&mut self, content: String) {
        if let Some(ref stream) = &self.stream {
            let mut s = stream;
            s.write(content.as_bytes()).unwrap();
        } else {
            eprintln!("Connection not established");
        }
    }

    pub fn read(&mut self) {
        let mut buffer = [0; 1024];
        if let Some(ref stream) = self.stream {
            let mut s = stream;
            s.read(&mut buffer).unwrap();
            eprintln!("{}", String::from_utf8_lossy(&buffer));
        } else {
            eprintln!("Connection not established");
        }
    }
}
