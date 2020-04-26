mod fane;
mod lamp;
mod lohse;
mod server;

use std::io::{Error};
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = 8196;
    let mut _local_lamp : lamp::Lamp = lamp::Lamp::new(false, true);

    let mut _local_server: server::Server = server::Server::new(port);
    _local_server.start()?;

    let mut lohse : lohse::Lohse = lohse::Lohse {
        _lamp: _local_lamp,
    };
    lohse.sync_lamp();

    let _fane: fane::Fane = fane::Fane {
        central_on: false
    };

    println!("Hello, lovol!");
    Ok(())
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
