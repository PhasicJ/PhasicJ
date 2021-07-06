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
    static ref EVENT_FORWARDING_CHANNEL: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

// This is meant to be used by event observation code within the agent to
// forward events out of the agent.
pub fn forward_event(event: ()) {
    EVENT_FORWARDING_CHANNEL.lock().unwrap().as_ref().unwrap().send(()).unwrap();
}

// This is meant to be called once during the agent's initial startup before
// the user's JVM application code has started to run.
pub fn start_event_forwarding_single_threaded_tokio_runtime() {
    // Initialize the event channel.
    let (tx, rx) = channel();
    {
        let mut data = EVENT_FORWARDING_CHANNEL.lock().unwrap();
        *data = Some(tx);
    }

    thread::spawn(move || {
        loop {
            let msg = rx.recv().unwrap();
            log::trace!("Observed an Event!");
        }
    });
}
