use tonic::{Request, Response, Status};

mod protos {
    tonic::include_proto!("phasicj.services.rtevents");
}

use protos::{
    recorder_server::Recorder,
    RtEvent, RecordEventsAck,
};

pub use protos::recorder_server::RecorderServer;

#[derive(Default)]
pub struct DaemonRecorder {}

impl DaemonRecorder {
    pub fn new() -> DaemonRecorder {
        DaemonRecorder::default()
    }
}

#[tonic::async_trait]
impl Recorder for DaemonRecorder {
    async fn record_events(
        &self,
        request: Request<RtEvent>
    ) -> Result<Response<RecordEventsAck>, Status> {
        log::info!("Got a `record_events()` request: {:?}", request);
        let reply = RecordEventsAck {
            message: "ack".into(),
        };
        log::info!("Sending an ACK back.");
        return Ok(Response::new(reply));
    }
}
