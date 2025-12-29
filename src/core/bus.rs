use crossbeam::channel::{unbounded, Sender, Receiver};
use crate::core::blade::Blade;

pub struct DataBus {
    tx: Sender<Blade>,
    rx: Receiver<Blade>,
}

impl DataBus {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        Self { tx, rx }
    }
    
    pub fn transmit(&self, packet: Blade) {
        let _ = self.tx.send(packet);
    }
}
