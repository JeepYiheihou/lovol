use tokio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::error::Error;

pub struct Server {
    loopback: String,
    port: u16
}

impl Server {
    pub fn new(port: u16) -> Server {
        return Server {
            loopback: String::from("127.0.0.1"),
            port: port
        };
    }

    pub async fn start(& mut self) -> Result<(), Box<dyn Error>> {

        let mut listener = TcpListener::bind(format!("{}:{}", self.loopback, self.port)).await?;

        loop {
            let (mut socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    let n = socket
                        .read(&mut buf)
                        .await
                        .expect("failed to read data from socket");

                    if n == 0 {
                        return;
                    }

                    socket
                        .write_all(&buf[0..n])
                        .await
                        .expect("failed to write data to socket");
                }
            });
        }
    }
}
