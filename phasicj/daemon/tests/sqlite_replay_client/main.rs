use phasicj_services_rtevents::recorder_client::RecorderClient;
use phasicj_services_rtevents::RtEvent;
use std::path::PathBuf;
use std::convert::TryFrom;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;
use clap::{AppSettings, Clap};
use rusqlite::Connection;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long)]
    pub database: String,

    #[clap(short, long)]
    pub socket: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let opts = Opts::parse();
    let socket_path = PathBuf::from(opts.socket);

    // Open database and create an iterator over all rows of the rtevents table.
    let db_conn = Connection::open(&opts.database)?;
    let mut stmt = db_conn.prepare("SELECT description FROM rtevents")?;
    let rtevents_iter = stmt.query_map([], |row| {
        Ok(row.get(0)?)
    })?;

    // This uri will be ignored because UNIX domain sockets do not use it.
    // TODO(dwtj): Is there a way to avoid it?
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| {
            // Connect to a UNIX domain socket.
            UnixStream::connect(socket_path.clone())
        }))
        .await?;

    let mut client = RecorderClient::new(channel);

    for rtevent in rtevents_iter {
        let request = tonic::Request::new(RtEvent {
            description: rtevent?,
        });
        let _response = client.record_events(request).await?;
    }

    Ok(())
}
