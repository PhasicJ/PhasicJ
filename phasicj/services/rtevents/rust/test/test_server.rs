#![cfg_attr(not(unix), allow(unused_imports))]

/// NOTE(dwtj): This code has been adapted from Tonic's [UNIX domain socket
/// example][1], which is MIT-licensed.
///
/// [1]: https://github.com/hyperium/tonic/blob/0583cff80f57ba071295416ee8828c3430851d0d/examples/src/uds/server.rs

use futures::TryFutureExt;
use std::path::Path;
use std::env;
#[cfg(unix)]
use tokio::net::UnixListener;
use tonic::{transport::Server, Request, Response, Status};

use phasicj_services_rtevents::{
    recorder_server::{Recorder, RecorderServer},
    RtEvent, RecordEventsAck,
};

#[derive(Default)]
pub struct MyRecorder {}

#[tonic::async_trait]
impl Recorder for MyRecorder {
    async fn record_events(
        &self,
        request: Request<RtEvent>,
    ) -> Result<Response<RecordEventsAck>, Status> {
        #[cfg(unix)]
        {
            // NOTE(dwtj): The `http::Extensions` type uses the runtime
            //  reflection support provided by `std::any` to implements a map
            //  from `TypeID`s to an instance of that type. In this case, it
            //  is used to hold our `UdsConnectInfo` custom type which we
            //  used to wrap/describe/point to a `Connected` UNIX domain socket.
            let conn_info = request.extensions().get::<unix::UdsConnectInfo>().unwrap();
            println!("Got a request {:?} with info {:?}", request, conn_info);
        }

        let reply = RecordEventsAck {
            message: "ack".into(),
        };
        Ok(Response::new(reply))
    }
}

fn get_socket_name() -> String {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    return args[1].clone();
}

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_socket_name();

    tokio::fs::create_dir_all(Path::new(&path).parent().unwrap()).await?;

    let handler = MyRecorder::default();

    let incoming = {
        let uds = UnixListener::bind(path)?;

        async_stream::stream! {
            while let item = uds.accept().map_ok(|(st, _)| unix::UnixStream(st)).await {
                yield item;
            }
        }
    };

    Server::builder()
        .add_service(RecorderServer::new(handler))
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}

#[cfg(unix)]
mod unix {
    pub use phasicj_services_util_tonic::uds::UnixStream;
    pub use phasicj_services_util_tonic::uds::UdsConnectInfo;
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}
