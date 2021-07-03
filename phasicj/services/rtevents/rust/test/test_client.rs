#![cfg_attr(not(unix), allow(unused_imports))]

/// NOTE(dwtj): This code has been adapted from Tonic's [UNIX domain socket
/// example][1].
///
/// [1]: https://github.com/hyperium/tonic/blob/0583cff80f57ba071295416ee8828c3430851d0d/examples/src/uds/client.rs

pub mod rtevents {
    tonic::include_proto!("phasicj.services.rtevents");
}

use rtevents::recorder_client::RecorderClient;
use rtevents::RtEvent;
use std::convert::TryFrom;
use std::env;
#[cfg(unix)]
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

fn get_socket_name() -> String {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    return args[1].clone();
}

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We will ignore this uri because uds do not use it
    // if your connector does use the uri it will be provided
    // as the request to the `MakeConnection`.
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            let path = get_socket_name();

            // Connect to a Uds socket
            UnixStream::connect(path)
        }))
        .await?;

    let mut client = RecorderClient::new(channel);

    let request = tonic::Request::new(RtEvent {
        description: "Tonic".into(),
    });

    let response = client.record_events(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}
