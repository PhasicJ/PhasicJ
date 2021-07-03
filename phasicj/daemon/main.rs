use std::path::Path;
use std::env;
use tokio::net::UnixListener;
use futures::TryFutureExt;
use rtevents::RecorderServer;
use tonic::transport::Server;

mod rtevents;

fn get_socket_name_from_cli_args() -> String {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    return args[1].clone();
}

// NOTE(dwtj): We don't use `#[tokio::main]` so that we can start up the logger
// before starting the Tokio runtime.
fn main() {
    env_logger::init();

    log::info!("PhasicJ Events Daemon started with PID {}", std::process::id());

    log::info!("Starting the Tokio runtime");
    let rt = tokio::runtime::Runtime::new().unwrap();
    log::info!("Tokio runtime started");

    rt.block_on(async {
        log::info!("Started root Tokio task");
        let socket_name = get_socket_name_from_cli_args();
        record_pj_rt_events_from_uds(&socket_name).await;
    });

    log::info!("Exited root Tokio task");
}

async fn record_pj_rt_events_from_uds(socket_name: &str) {
    use phasicj_services_util_tonic::uds::UnixStream;

    let socket_path = Path::new(&socket_name);

    // Ensure that the UDS's parent directory exists.
    tokio::fs::create_dir_all((&socket_path).parent().unwrap()).await.unwrap();

    // Bind to the socket. For each connection from this socket,
    // - wrap this connection in our custom `UnixStream` type (which implements
    //   Tonic's `Connected` trait)
    // - then yield this wrapped connection to the stream.
    let incoming_connections_stream = {
        log::info!("Binding to a UNIX domain socket at path {:?}", socket_path);
        let uds = UnixListener::bind(socket_path).unwrap();

        async_stream::stream! {
            while let conn = uds.accept().map_ok(|(s, _)| UnixStream(s)).await {
                log::info!("Accepted a connection from UNIX domain socket");
                yield conn;
            }
        }
    };

    // Make a recorder which will handle gRPC calls from any of the connections
    // yielded by the above stream.
    let recorder = rtevents::DaemonRecorder::new();
    Server::builder()
        .add_service(RecorderServer::new(recorder))
        .serve_with_incoming(incoming_connections_stream)
        .await
        .unwrap();
}
