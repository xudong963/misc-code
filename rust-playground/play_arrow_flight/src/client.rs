use crate::flight::flight_action::FlightAction;
use crate::flight::flight_client::FlightClient;
use arrow_format::flight::service::flight_service_client::FlightServiceClient;
use tonic::transport::{Channel, Uri};

mod flight;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9093".to_string();
    let channel_builder = Channel::builder(format!("http://{addr}").parse::<Uri>().unwrap());
    let channel = channel_builder.connect().await.unwrap();
    let mut flight_client = FlightClient::new(FlightServiceClient::new(channel));
    let flight_action = FlightAction::Action1;
    let res = flight_client.do_action(flight_action).await.unwrap();
    dbg!(std::str::from_utf8(&res).unwrap());
}
