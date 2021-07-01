#![cfg_attr(not(unix), allow(unused_imports))]

use futures::TryFutureExt;
use std::path::Path;
use std::env;
#[cfg(unix)]
use tokio::net::UnixListener;
use tonic::{transport::Server, Request, Response, Status};

pub mod rtevents {
    tonic::include_proto!("phasicj.services.rtevents");
}

use rtevents::{
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
    use std::{
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tonic::transport::server::Connected;

    #[derive(Debug)]
    pub struct UnixStream(pub tokio::net::UnixStream);

    impl Connected for UnixStream {
        type ConnectInfo = UdsConnectInfo;

        fn connect_info(&self) -> Self::ConnectInfo {
            UdsConnectInfo {
                peer_addr: self.0.peer_addr().ok().map(Arc::new),
                peer_cred: self.0.peer_cred().ok(),
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct UdsConnectInfo {
        pub peer_addr: Option<Arc<tokio::net::unix::SocketAddr>>,
        pub peer_cred: Option<tokio::net::unix::UCred>,
    }

    impl AsyncRead for UnixStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for UnixStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}

#[cfg(not(unix))]
fn main() {
    panic!("The `uds` example only works on unix systems!");
}
