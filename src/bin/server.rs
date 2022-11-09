use sigma::shared::cli::*;
use message_io::node::{self};
use message_io::network::{NetEvent, Transport, Endpoint};
use std::collections::HashMap;
use std::net::{ToSocketAddrs};

struct ClientInfo(usize);

fn main() {
    let addr = ("0.0.0.0", 3001).to_socket_addrs().unwrap().next().unwrap();
    let (handler, listener) = node::split::<()>();
    let mut clients: HashMap<Endpoint, ClientInfo> = HashMap::new();
    handler.network().listen(Transport::FramedTcp, addr).unwrap();
    listener.for_each(move |event| {
        match event.network() {
            NetEvent::Connected(_,_) => {}, 
            NetEvent::Accepted(_,_) => {},
            NetEvent::Message(_,_) => {},
            NetEvent::Disconnected(_) => {}
        }
    });

}
