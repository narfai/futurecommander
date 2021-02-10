use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use futurecommander_proto::vfs::{
    virtual_file_system_server::{VirtualFileSystem,VirtualFileSystemServer},
//    Entry,
    ListDirectoryRequest,
    ListDirectoryResponse,
//    CreateNodeRequest,
//    CreateNodeResponse,
    RemoveNodeRequest,
    RemoveNodeResponse,
//    CopyNodeRequest,
//    CopyNodeResponse,
//    MoveNodeRequest,
//    MoveNodeResponse,
    RequestStatus,
    ResponseStatus,
};

#[derive(Default)]
pub struct Daemon {}

#[tonic::async_trait]
impl VirtualFileSystem for Daemon {
    type RemoveNodeStream = mpsc::Receiver<Result<RemoveNodeResponse, Status>>;
    async fn remove_node(
        &self,
        request: Request<tonic::Streaming<RemoveNodeRequest>>,
    ) -> Result<Response<Self::RemoveNodeStream>, Status> {
        let mut streamer = request.into_inner();
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(req) = streamer.message().await.unwrap(){
                println!("Got a delete request {:?}", req);
                tx.send(Ok(RemoveNodeResponse {
                    status: ResponseStatus::Processing as i32,
                    message: Some(format!("{:?}/A", req.path)),
                    error: None
                })).await;
                tx.send(Ok(RemoveNodeResponse {
                    status: ResponseStatus::Processing as i32,
                    message: Some(format!("{:?}/B", req.path)),
                    error: None
                }))
                .await;
                tx.send(Ok(RemoveNodeResponse {
                    status: ResponseStatus::Done as i32,
                    message: None,
                    error: None
                }))
                .await;
            }
        });
        Ok(Response::new(rx))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let daemon = Daemon::default();

    let ser = VirtualFileSystemServer::new(daemon);
    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(ser)
        .serve(addr)
        .await?;

    Ok(())
}
