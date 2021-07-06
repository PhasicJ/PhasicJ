use std::path::Path;
use tokio::net::UnixListener;
use futures::TryFutureExt;
use rtevents::SqliteRecorder;
use tonic::transport::Server;
use clap::Clap;
use phasicj_services_rtevents::recorder_server::RecorderServer;

mod rtevents;
mod cli;

// NOTE(dwtj): We don't use `#[tokio::main]` so that we can start up the logger
// before starting the Tokio runtime.
fn main() {
    env_logger::init();

    log::info!("PhasicJ Events Daemon started with PID {}", std::process::id());

    log::info!("Starting the Tokio runtime");
    let rt = tokio::runtime::Runtime::new().unwrap();
    log::info!("Tokio runtime started");

    let opts = cli::Opts::parse();
    log::info!("CLI options parsed: {:?}", opts);

    rt.block_on(async {
        log::info!("Started root Tokio task");
        let socket = Path::new(&opts.socket);
        let database = Path::new(&opts.database);
        record_pj_rt_events_from_uds(socket, database).await;
        log::info!("Ending root Tokio task");
    });

    log::info!("Exited root Tokio task");
}

async fn record_pj_rt_events_from_uds(socket: &Path, database: &Path) {
    use phasicj_services_util_tonic::uds::UnixStream;

    // Ensure that the UDS's parent directory exists.
    tokio::fs::create_dir_all(socket.parent().unwrap()).await.unwrap();

    // Bind to the socket. For each connection from this socket,
    // - wrap this connection in our custom `UnixStream` type (which implements
    //   Tonic's `Connected` trait)
    // - then yield this wrapped connection to the stream.
    let incoming_connections_stream = {
        log::info!("Binding to a UNIX domain socket at path {:?}", socket);
        let uds = UnixListener::bind(socket).unwrap();

        async_stream::stream! {
            while let conn = uds.accept().map_ok(|(s, _)| UnixStream(s)).await {
                log::info!("Accepted a connection from UNIX domain socket");
                yield conn;
            }
        }
    };

    // Make a recorder which will handle gRPC calls from any of the connections
    // yielded by the above stream.
    let recorder = SqliteRecorder::new(database).unwrap();
    Server::builder()
        .add_service(RecorderServer::new(recorder))
        .serve_with_incoming(incoming_connections_stream)
        .await
        .unwrap();
}
