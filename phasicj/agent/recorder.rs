// This module is responsible for forwarding observed application events to the
// daemon via gRPC over UDP.

use std::sync::mpsc::Sender;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::thread;
use std::sync::mpsc::channel;
use tokio::runtime::{Runtime, Builder};
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri, Channel};
use tower::service_fn;
use phasicj_services_rtevents::{
    recorder_client::RecorderClient,
    RtEvent,
};
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

const NUM_WORKER_THREADS: usize = 2;
const WORKER_THREAD_NAME: &'static str = "phasicj-agent-event-forwarder-worker";

lazy_static! {
    static ref RUNTIME: Runtime = {
        log::trace!("Starting a Tokio runtime for exclusive use by the agent's event forwarder");
        Builder::new_multi_thread()
            .worker_threads(NUM_WORKER_THREADS)
            .thread_name(WORKER_THREAD_NAME)
            .build()
            .unwrap()
        };
    static ref EVENT_FORWARDER: EventForwarder = EventForwarder {
        recorder_client: Mutex::new(None),
    };
}

pub fn install_event_forwarder_singleton(daemon_uds_path: &Path) {
    log::trace!("Installing event forwarder singleton");

    if EVENT_FORWARDER.recorder_client.lock().unwrap().is_none() {
        panic!("An event forwarder has already been installed");
    }

    // Make an owned copy of the path that can be moved into the async block.
    let daemon_uds_pathbuf = daemon_uds_path.to_path_buf();

    let recorder_client: RecorderClient<Channel> = RUNTIME.block_on(async move {
        // NOTE(dwtj): According to Tonic's `uds` example, this URI will be
        // ignored because UNIX domain sockets do not use it.
        // TODO(dwtj): Is there a way to avoid having a URI at all?

        log::info!(
            "Connecting to the daemon via UNIX domain socket at path {}",
            daemon_uds_path.to_str().unwrap(),
        );

        let channel = Endpoint::try_from("http://[::]:50051")
            .unwrap()
            .connect_with_connector(service_fn(move |_: Uri| {
                UnixStream::connect(daemon_uds_pathbuf.clone())
            }))
            .await
            .unwrap();

        RecorderClient::new(channel)
    });

    *EVENT_FORWARDER.recorder_client.lock().unwrap() = Some(recorder_client);
}

// This is meant to be used by event observation code within the agent to
// forward events out of the agent. Events passed to this function will be
// forwarded out of the agent to the daemon to be recorded. The `EventForwarder`
// singleton will be used to send these events via gRPC over UNIX domain socket.
// This will fail if the `EventForwarder` singleton
pub fn forward_event(event: ()) {
    log::trace!("Forwarded an event: {:?}", event);

    RUNTIME.spawn(async move {
        log::trace!("Forwarded an event into Tokio runtime: {:?}", event);
        let request = tonic::Request::new(RtEvent {
            description: "()".to_string(),
        });
        let result = EVENT_FORWARDER.recorder_client
            .lock()
            .unwrap()
            .unwrap()
            .record_events(request)
            .await
            .unwrap();
    });

}

/// Used to store the state of our singleton.
struct EventForwarder {
    recorder_client: Mutex<Option<RecorderClient<Channel>>>,
}
