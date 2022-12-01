use devices::sensor_client::SensorClient;
use devices::SensorRequest;

pub mod devices {
    tonic::include_proto!("devices");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SensorClient::connect("http://127.0.0.1:8086").await?;
    let request = tonic::Request::new(
        SensorRequest {
            device: "Device2".into(),//to_owned(),
        }
    );
    let response = client.send(request).await?;
    let resp = response.into_inner();
    println!("RESPONSE={:#?}", resp.message);
    Ok(())
}


