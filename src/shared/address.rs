use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub id: u64,
    pub host: String,
    pub port: u16,
    pub support: Vec<ConnectionType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ConnectionType {
    TCP,
    UDP,
    UDPMulticast,
    WebSocket,
}
