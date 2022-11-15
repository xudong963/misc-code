use crate::flight::flight_action::FlightAction;
use arrow_format::flight::data::Action;
use arrow_format::flight::service::flight_service_client::FlightServiceClient;
use tonic::transport::Channel;
use tonic::{Request, Status};

pub struct FlightClient {
    inner: FlightServiceClient<Channel>,
}

impl FlightClient {
    pub fn new(inner: FlightServiceClient<Channel>) -> Self {
        Self { inner }
    }

    pub async fn do_action(&mut self, action: FlightAction) -> Result<Vec<u8>, Status> {
        let action: Action = action.try_into()?;
        let request = Request::new(action);
        let response = self.inner.do_action(request).await?;
        match response.into_inner().message().await? {
            Some(response) => Ok(response.body),
            None => Ok(vec![0]),
        }
    }
}
