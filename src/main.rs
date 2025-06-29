use tonic::{transport::Server, Request, Response, Status};
use comand::comand_service_server::{ComandService, ComandServiceServer};
use comand::{CommandRequest, CommandResponse, GetCommandRequest, ListCommandsRequest};
use sqlx::PgPool;
use tokio_stream::wrappers::ReceiverStream;
use tracing::info;

pub mod comand {
    tonic::include_proto!("comand");
}

mod db;

#[derive(Debug)]
struct MyComandService {
    pool: PgPool,
}

#[tonic::async_trait]
impl ComandService for MyComandService {
    async fn execute_command( &self, request: Request<CommandRequest> ) -> Result<Response<CommandResponse>, Status> {

        let command = request.into_inner().command;

        if command.is_empty() {
            return Err(Status::invalid_argument("Command cannot be empty"));
        }

        let output = format!("Command executed: {}", command);

        let id = db::save_command(&self.pool, &command, &output)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;
        
        info!("Saved command: id={}, command={}", id, command);
        Ok(Response::new(CommandResponse {
            id,
            command,
            output,
            created_at: chrono::Utc::now().to_rfc3339(),
        }))
    }

    async fn get_command( &self, request: Request<GetCommandRequest>) -> Result<Response<CommandResponse>, Status> {

        let id = request.into_inner().id;

        let command = db::get_command(&self.pool, id)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;
        
        match command {
            Some(cmd) => {
                info!("Retrieved command: id={}", id);
                Ok(Response::new(CommandResponse {
                    id: cmd.id,
                    command: cmd.command,
                    output: cmd.output,
                    created_at: cmd.created_at.to_rfc3339(),
                }))
            }
            None => Err(Status::not_found("Command not found")),
        }
    }

    type ListCommandsStreamStream = ReceiverStream<Result<CommandResponse, Status>>;

    async fn list_commands_stream( &self, _request: Request<ListCommandsRequest> ) -> Result<Response<Self::ListCommandsStreamStream>, Status> {

        let commands = db::list_commands(&self.pool)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;
        
        let (tx, rx) = tokio::sync::mpsc::channel(4);

        for cmd in &commands {
            tx.send(Ok(CommandResponse {
                id: cmd.id,
                command: cmd.command.clone(),
                output: cmd.output.clone(),
                created_at: cmd.created_at.to_rfc3339(),
            }))
            .await
            .map_err(|e| Status::internal(format!("Stream error: {}", e)))?;
        }
        info!("Streaming {} commands", commands.len());
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let pool = db::init_db_pool().await?;
    let addr = "127.0.0.1:50051".parse()?;
    let comand_service = MyComandService { pool };

    info!("Server running on {}", addr);
    Server::builder()
        .add_service(ComandServiceServer::new(comand_service))
        .serve(addr)
        .await?;

    Ok(())
}