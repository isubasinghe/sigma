use tokio::sync::mpsc;

pub struct Server {
   receiver: Option<mpsc::Receiver<u32>>
}

impl Server {
    async fn new() {
    }
}
