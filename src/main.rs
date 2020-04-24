mod lamp;
mod server;

use std::io::{Read, Error};

fn main() -> Result<(), Error> {
    let port = 8196;
    let mut _local_lamp : lamp::Lamp = lamp::Lamp::new(false, false);
    println!("Hello, lovol! before {} ", _local_lamp.is_on);
    _local_lamp.turn_on();

    let mut _local_server: server::Server = server::Server::new(port);
    _local_server.start()?;

    println!("Hello, lovol! after {} ", _local_lamp.is_on);

    Ok(())
}
