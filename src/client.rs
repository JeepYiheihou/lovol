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

    pub fn connect(& mut self) {
        match TcpStream::connect(format!("{}:{}", self.target, self.port)) {
            Ok(stream) => self.stream = Some(stream),
            Err(e) => eprintln!("Error connecting to server: {}", e),
        }
    }

    pub fn send_and_wait_for_response(& mut self) -> std::io::Result<()> {
        if let Some(ref stream) = self.stream {
            stream.write(a)?;
            Ok(())
        } else {
            eprintln!("Connection not established");
            Ok(())
        }
    }
}
