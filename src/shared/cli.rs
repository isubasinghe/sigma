use crate::shared::address::Address;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use toml;

use super::address::ConnectionType;

#[derive(Parser, Debug)]
#[command(name = "Sigma Client")]
#[command(author = "Isitha Subasinghe")]
#[command(author, version, about, long_about = None)]
pub struct ClientArgs {
    #[arg(short = 'i', long = "id", value_name = "ID")]
    client_id: u32,
    #[arg(short = 'p', long = "port", value_name = "PORT")]
    port: u16,
    #[arg(short = 'F', long = "peer-file", value_name = "PEERS FILE")]
    peer_file: String,
}

impl ClientArgs {
    pub fn parse_cli() -> ClientArgs {
        ClientArgs::parse()
    }

    pub fn to_client_data() -> io::Result<ClientData> {
        let cargs = ClientArgs::parse();
        let contents = fs::read_to_string(cargs.peer_file)?;
        let peers: PeerData = toml::from_str(&contents)?;
        Ok(ClientData {
            peers,
            id: cargs.client_id,
            port: cargs.port,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeerData {
    peers: Vec<Address>,
}

pub struct ClientData {
    pub peers: PeerData,
    pub id: u32,
    pub port: u16,
}

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(name = "Sigma Server")]
#[command(author = "Isitha Subasinghe")]
#[command(author, version, about, long_about = None)]
pub struct ServerArgs {
    #[arg(short = 'i', long = "id", value_name = "ID")]
    server_id: u32,
    #[arg(short = 'F', long = "peer-file", value_name = "PEERS FILE")]
    peer_file: String,
}

impl ServerArgs {
    pub fn parse_cli() -> ServerArgs {
        ServerArgs::parse()
    }
    pub fn to_server_data() -> io::Result<ServerData> {
        let sargs = ServerArgs::parse();
        let contents = fs::read_to_string(sargs.peer_file)?;
        let peers: PeerData = toml::from_str(&contents)?;

        todo!()
    }
}

pub struct ServerData {
    pub peers: PeerData,
    pub id: u32,
    pub port: u16,
}
