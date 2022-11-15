use crate::flight::flight_action::FlightAction;
use arrow_format::flight::data::{
    Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
    HandshakeRequest, HandshakeResponse, PutResult, SchemaResult, Ticket,
};
use arrow_format::flight::service::flight_service_server::FlightService;
use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Request, Response, Status, Streaming};

pub struct DemoFlightService {}

impl DemoFlightService {
    pub fn new() -> Self {
        Self {}
    }
}

pub type FlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, tonic::Status>> + Send + Sync + 'static>>;
type StreamReq<T> = Request<Streaming<T>>;

#[async_trait::async_trait]
impl FlightService for DemoFlightService {
    type HandshakeStream = FlightStream<HandshakeResponse>;

    async fn handshake(
        &self,
        _request: StreamReq<HandshakeRequest>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        todo!()
    }

    type ListFlightsStream = FlightStream<FlightInfo>;

    async fn list_flights(
        &self,
        _request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        todo!()
    }

    async fn get_flight_info(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        todo!()
    }

    async fn get_schema(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        todo!()
    }

    type DoGetStream = FlightStream<FlightData>;

    async fn do_get(
        &self,
        _request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        todo!()
    }

    type DoPutStream = FlightStream<PutResult>;

    async fn do_put(
        &self,
        _request: StreamReq<FlightData>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        todo!()
    }

    type DoExchangeStream = FlightStream<FlightData>;

    async fn do_exchange(
        &self,
        _request: StreamReq<FlightData>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        todo!()
    }

    type DoActionStream = FlightStream<arrow_format::flight::data::Result>;

    async fn do_action(
        &self,
        request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        let request = request.into_inner();
        let flight_action = request.try_into()?;
        let action_result = match flight_action {
            FlightAction::Null => arrow_format::flight::data::Result { body: b"".to_vec() },
            FlightAction::Action1 => arrow_format::flight::data::Result {
                body: b"action1".to_vec(),
            },
            FlightAction::Action2 => arrow_format::flight::data::Result {
                body: b"action2".to_vec(),
            },
            FlightAction::Action3 => arrow_format::flight::data::Result {
                body: b"action3".to_vec(),
            },
        };
        Ok(Response::new(Box::pin(tokio_stream::once(Ok(
            action_result,
        )))))
    }

    type ListActionsStream = FlightStream<ActionType>;

    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        todo!()
    }
}
