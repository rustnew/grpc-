use tonic::transport::Channel;
use comand::comand_service_client::ComandServiceClient;
use comand::CommandRequest;

pub mod comand {
    tonic::include_proto!("comand");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ComandServiceClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(CommandRequest {
        command: "test_command".into(),
    });

    let response = client.execute_command(request).await?;

    println!("Response: {:?}", response.into_inner().output);
    Ok(())
}