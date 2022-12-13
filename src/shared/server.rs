use crate::shared::proto_capnp::pair;
use arc_swap::ArcSwapOption;
use dashmap::DashSet;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

#[derive(Hash)]
pub struct Id(u64, u64);

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 && self.0 == other.0
    }
}

impl Eq for Id {}

struct Tombstone {
    entries: DashSet<Id>,
}

impl Tombstone {
    pub fn new() -> Tombstone {
        Tombstone {
            entries: DashSet::new(),
        }
    }
}

impl Id {
    pub fn new(client_id: u64, timestamp: u64) -> Id {
        Id(client_id, timestamp)
    }
}

type Owner = (u64, u64);

pub struct Server<T> {
    receiver: mpsc::Receiver<T>,
    owner: Mutex<Option<Owner>>,
}

const SERVER_BUFFER_SIZE: usize = 2048;

impl<T> Server<T> {
    pub fn new() -> (Server<T>, mpsc::Sender<T>) {
        let (tx, rx) = mpsc::channel(SERVER_BUFFER_SIZE);

        (
            Server {
                receiver: rx,
                owner: Mutex::new(None),
            },
            tx,
        )
    }

    pub async fn release(&mut self) {}

    pub async fn yield_lock(&mut self) {}

    pub async fn try_lock(&mut self, id: Id) {
        match self.owner.lock().await.as_mut() {
            None => {}
            Some(owner) => {}
        }
    }

    pub async fn inquiry(&mut self) {}
}
