use comand::comand_service_client::ComandServiceClient;
use comand::{CommandRequest, GetCommandRequest, ListCommandsRequest};
use tracing::info;

pub mod comand {
    tonic::include_proto!("comand");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt::init();

    let mut client = ComandServiceClient::connect("http://127.0.0.1:50051").await?;

    // Test ExecuteCommand
    let request = tonic::Request::new(CommandRequest {
        command: "test_command".into(),
    });
    
    let response = client.execute_command(request).await?;
    info!("ExecuteCommand Response: {:?}", response.into_inner());

    // Test GetCommand
    let request = tonic::Request::new(GetCommandRequest { id: 1 });
    let response = client.get_command(request).await?;
    info!("GetCommand Response: {:?}", response.into_inner());

    // Test ListCommandsStream
    let request = tonic::Request::new(ListCommandsRequest {});
    let mut stream = client.list_commands_stream(request).await?.into_inner();
    while let Some(response) = stream.message().await? {
        info!("ListCommandsStream Response: {:?}", response);
    }

    Ok(())
}