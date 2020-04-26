use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio;
use tokio::net::UdpSocket;

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

    async fn run(& mut self) -> Result<(), Error> {
        let Server {
            mut socket,
            mut buf,
            mut to_send,
        } = self;

        let mut listener = TcpListener::bind(&addr).await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let (mut socket, _) = listener.accept().await?;

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
