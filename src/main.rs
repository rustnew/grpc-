use tonic::{transport::Server, Request, Response, Status};
use comand::comand_service_server::{ComandService, ComandServiceServer};
use comand::{CommandRequest, CommandResponse};

pub mod comand {
    tonic::include_proto!("comand");
}

#[derive(Debug)]
struct MyComandService;

#[tonic::async_trait]
impl ComandService for MyComandService {
    async fn execute_command(
        &self,
        request: Request<CommandRequest>,
    ) -> Result<Response<CommandResponse>, Status> {
        let command = request.into_inner().command;
        let reply = CommandResponse {
            output: format!("Command executed: {}", command),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let comand_service = MyComandService;

    println!("Server running on {}", addr);

    Server::builder()
        .add_service(ComandServiceServer::new(comand_service))
        .serve(addr)
        .await?;

    Ok(())
}