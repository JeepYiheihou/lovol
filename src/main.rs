mod fane;
mod lamp;
mod lohse;
mod server;

use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = 8196;
    let mut _local_lamp : lamp::Lamp = lamp::Lamp::new(false, true);

    let mut lohse : lohse::Lohse = lohse::Lohse {
        _lamp: _local_lamp,
    };
    lohse.sync_lamp();

    let _fane: fane::Fane = fane::Fane {
        central_on: false
    };

    let mut _local_server: server::Server = server::Server::new(port);
    _local_server.start().await?;
    Ok(())
}
