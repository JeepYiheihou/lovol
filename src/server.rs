use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::{Read, Error};

pub struct Server {
    loopback: Ipv4Addr,
    port: u16
}

impl Server {
    pub fn new(port: u16) -> Server {
        return Server {
            loopback: Ipv4Addr::new(127, 0, 0, 1),
            port: port
        };
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let socket = SocketAddrV4::new(self.loopback, self.port);
        let listener = TcpListener::bind(socket)?;
        println!("Listening on {}, access this port to end the program", self.port);
        loop {
            let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
            println!("Connection received! {:?} is sending data.", addr);
            let mut input = String::new();
            let _ = tcp_stream.read_to_string(&mut input)?;
            println!("{:?} says {}", addr, input);
        }
    }
}
