// This module is responsible for forwarding observed application events to the
// daemon via gRPC over UDP.

use std::sync::mpsc::Sender;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::thread;
use std::sync::mpsc::channel;

lazy_static! {
    // TODO(): Events sent into this channel will be forwarded to the daemon via
    // gRPC over UDP.
    pub static ref EVENT_FORWARDING_CHANNEL: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

pub fn start_forwarding_worker_thread() {
    // Initialize the channel.
    let (tx, rx) = channel();
    {
        let mut data = EVENT_FORWARDING_CHANNEL.lock().unwrap();
        *data = Some(tx);
    }

    thread::spawn(move || {
        loop {
            let msg = rx.recv().unwrap();
            log::trace!("JVM Monitor Event!");
        }
    });
}
