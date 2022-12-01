use std::time::Duration;

use tonic::{Request, Response, Status, transport::Server};

use devices::{SensorRequest, SensorResponse};
use devices::sensor_server::{Sensor, SensorServer};
use rust-grpc-example::services::data;

pub mod devices {
    tonic::include_proto!("devices");
}

#[derive(Debug, Default)]
pub struct SensorService {}

#[tonic::async_trait]
impl Sensor for SensorService {
    async fn send(&self, request: Request<SensorRequest>) -> Result<Response<SensorResponse>, Status> {
        let req = request.into_inner();
        println!("Got a request: {:#?}", req.device);
        let res = data::getbydevice(req.device).await.unwrap();
        let reply = SensorResponse {
            successful: true,
            message: format!("Device : {} , Date : {} , Value : {} ", res.device, res.date, res.value),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8086".parse()?;
    let service = SensorService::default();
    println!("Server is running...");
    Server::builder()
            .timeout(Duration::from_secs(10))
            .add_service(SensorServer::new(service))
            .serve(addr)
            .await?;
    Ok(())
}


