// This module is responsible for forwarding observed application events to the
// daemon via gRPC over UDP.
//
// I found some guidance for how to design this from [this section][1] of the
// Tokio tutorial.
//
// [1]: https://tokio.rs/tokio/tutorial/bridging#sending-messages

use lazy_static::lazy_static;
use tokio::sync::mpsc;
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

const EVENT_CHANNEL_SIZE: usize = 128;
const NUM_WORKER_THREADS: usize = 2;
const WORKER_THREAD_NAME: &'static str = "phasicj-agent-event-forwarder-worker";
const DAEMON_SOCKET_PATH: &'static str = "pjeventsd.S";

// pub fn install_static_event_forwarder(daemon_uds_path: &Path) {
//     log::trace!("Installing event forwarder as singleton");

//     if EVENT_FORWARDER.is_some() {
//         panic!("An event forwarder has already been installed");
//     }

//     *EVENT_FORWARDER = Some(EventForwarder::new(daemon_uds_path));
// }

// This is meant to be used by event observation code within the agent to
// forward events out of the agent. Events passed to this function will be
// forwarded out of the agent to the daemon to be recorded. The `EventForwarder`
// singleton will be used to send these events via gRPC over UNIX domain socket.
// This will fail if the `EventForwarder` singleton
pub fn forward_event(event: ()) {
    log::trace!("Forwarding an event: {:?}", event);
    EVENT_FORWARDER.as_ref().unwrap().forward_event(event);
}

lazy_static! {
    static ref EVENT_FORWARDER: Option<EventForwarder> =
        Some(EventForwarder::new(Path::new(DAEMON_SOCKET_PATH)));
}

async fn new_recorder_client(daemon_uds_path: &Path) -> RecorderClient<Channel> {
    log::info!(
        "Connecting to the daemon via UNIX domain socket at path {}",
        daemon_uds_path.to_str().unwrap(),
    );
    let daemon_uds_path_buf = daemon_uds_path.to_path_buf();
    // NOTE(dwtj): According to Tonic's `uds` example, this URI will be ignored
    // because UNIX domain sockets do not use it.
    // TODO(dwtj): Is there a way to avoid having a URI at all?
    let channel = Endpoint::try_from("http://[::]:50051")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            UnixStream::connect(daemon_uds_path_buf.clone())
        }))
        .await
        .unwrap();

    RecorderClient::new(channel)
}

/// Used to store the state of our singleton.
struct EventForwarder {
    tx: mpsc::Sender<()>,
}

impl EventForwarder {
    fn new(daemon_uds_path: &Path) -> EventForwarder {
        log::trace!("Starting a Tokio runtime for exclusive use an Agent event forwarder");
        let runtime = Builder::new_multi_thread()
            .worker_threads(NUM_WORKER_THREADS)
            .thread_name(WORKER_THREAD_NAME)
            .build()
            .unwrap();

        let daemon_uds_pathbuf = daemon_uds_path.to_path_buf();
        let (tx, mut rx) = mpsc::channel(EVENT_CHANNEL_SIZE);
        runtime.spawn(async move {
            let mut client = new_recorder_client(&daemon_uds_pathbuf).await;
            while let event = rx.recv().await {
                log::trace!("Forwarded an event into Tokio runtime: {:?}", event);
                let request = tonic::Request::new(RtEvent {
                    description: "()".to_string(),
                });
                let response = client.record_events(request).await;
            }
        });

        EventForwarder {
            tx,
        }
    }

    fn forward_event(&self, event: ()) {
        self.tx.blocking_send(event);
    }
}
