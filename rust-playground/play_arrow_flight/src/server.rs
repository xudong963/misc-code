mod flight;

use arrow_format::flight::service::flight_service_server::FlightServiceServer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::time::sleep;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

use crate::flight::flight_service::DemoFlightService;

struct DemoServer {}

impl DemoServer {
    pub fn new() -> Self {
        Self {}
    }

    async fn listen_tcp(&self, addr: SocketAddr) -> Result<TcpListenerStream, ()> {
        let listener = TcpListener::bind(addr).await.unwrap();
        Ok(TcpListenerStream::new(listener))
    }

    async fn start_with_incoming(&self, listener_stream: TcpListenerStream) -> Result<(), ()> {
        let demo_flight_service = DemoFlightService::new();
        let mut builder = Server::builder();
        let server = builder
            .add_service(FlightServiceServer::new(demo_flight_service))
            .serve_with_incoming(listener_stream);
        tokio::spawn(server);
        Ok(())
    }

    async fn start(&self, addr: SocketAddr) -> Result<(), ()> {
        let listener_stream = self.listen_tcp(addr).await?;
        self.start_with_incoming(listener_stream).await?;
        Ok(())
    }

    async fn shutdown(&self) {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let addr = "127.0.0.1:9093".to_string();
    let server = DemoServer::new();
    server.start(addr.parse().unwrap()).await?;
    sleep(std::time::Duration::from_secs(10000)).await;
    Ok(())
}
