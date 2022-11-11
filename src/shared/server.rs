use tokio::sync::mpsc;
use crate::shared::proto_capnp::pair;

pub struct Server<T> {
    receiver: mpsc::Receiver<T>,
    owner: Option<(u64, u64)>,
}

const SERVER_BUFFER_SIZE: usize = 2048;

impl<T> Server<T> {
    pub fn new() -> (Server<T>, mpsc::Sender<T>) {
        let (tx, rx) = mpsc::channel(SERVER_BUFFER_SIZE);

        (
            Server {
                receiver: rx,
                owner: None,
            },
            tx,
        )
    }
}
