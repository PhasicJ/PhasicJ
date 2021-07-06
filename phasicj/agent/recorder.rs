// This module is responsible for forwarding observed application events to the
// daemon via gRPC over UDP.

use std::sync::mpsc::Sender;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::thread;
use std::sync::mpsc::channel;
use tokio::runtime;

lazy_static! {
    // TODO(): Events sent into this channel will be forwarded to the daemon via
    // gRPC over UDP.
    static ref EVENT_FORWARDING_RUNTIME: runtime::Runtime =
        runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("grpc_recorder_client")
            .build()
            .unwrap();
}

// This is meant to be used by event observation code within the agent to
// forward events out of the agent.
pub fn forward_event(event: ()) {
    EVENT_FORWARDING_RUNTIME.spawn(async move {
        log::trace!("Forwarded an event: {:?}", event)
    });
}
