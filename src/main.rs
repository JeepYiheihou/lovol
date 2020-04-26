mod lamp;
mod lohse;
mod server;

use std::io::{Error};

fn main() -> Result<(), Error> {
    let port = 8196;
    let mut _local_lamp : lamp::Lamp = lamp::Lamp::new(false, true);

    let mut _local_server: server::Server = server::Server::new(port);
    _local_server.start()?;

    let mut lohse : lohse::Lohse = lohse::Lohse {
        _lamp: _local_lamp,
    };
    lohse.sync_lamp();

    println!("Hello, lovol!");

    Ok(())
}