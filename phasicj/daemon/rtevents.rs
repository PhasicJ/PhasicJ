use std::path::{Path};
use std::sync::Mutex;
use tonic::{Request, Response, Status};
use rusqlite::{params, Connection};

mod protos {
    tonic::include_proto!("phasicj.services.rtevents");
}

use protos::{
    recorder_server::Recorder,
    RtEvent, RecordEventsAck,
};

pub use protos::recorder_server::RecorderServer;

pub struct SqliteRecorder {
    database_connection: Mutex<Connection>,
}

impl SqliteRecorder {
    pub fn new(database: &Path) -> rusqlite::Result<SqliteRecorder> {
        let database_connection = Connection::open(database)?;
        database_connection.execute(
            "CREATE TABLE rtevents (
                id            INTEGER PRIMARY KEY,
                description   TEXT
                )",
                []
        )?;
        Ok(SqliteRecorder {
            database_connection: Mutex::new(database_connection),
        })
    }
}

// TODO(dwtj): WARNING! This design has very bad concurrency. Different
// `record_events` gRPC calls will all contend for a single SQLite `Connection`
// object and they will block their threads from the Tokio runtime's thread pool
// while doing so. This is exactly what one is not supposed to do when using
// async/await and a thread-pool-based Tokio runtime. I've only implemented it
// this way temporarily to begin prototyping our use of SQLite via `rusqlite`.
#[tonic::async_trait]
impl Recorder for SqliteRecorder {
    async fn record_events(
        &self,
        request: Request<RtEvent>
    ) -> Result<Response<RecordEventsAck>, Status> {
        log::info!("Got a `record_events()` request: {:?}", request);
        {
            let ev_descr: &str = &(request.into_inner().description);
            let conn = self.database_connection.lock().unwrap();
            conn.execute(
                "INSERT INTO rtevents (description) VALUES (?1)",
                params![ev_descr]
            ).unwrap();
        }
        log::info!("Got the database connection lock");
        let reply = RecordEventsAck {
            message: "ack".into(),
        };
        log::info!("Sending an ACK back.");
        return Ok(Response::new(reply));
    }
}
