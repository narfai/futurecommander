use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use futurecommander_proto::vfs::{
    virtual_file_system_server::{VirtualFileSystem,VirtualFileSystemServer},
//    Entry,
//    ListDirectoryRequest,
//    ListDirectoryResponse,
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
                    status: ResponseStatus::Done as i32,
                    error: None
                }))
                .await;
            }
        });
        Ok(Response::new(rx))
    }
}

fn interceptor(req:Request<()>)->Result<Request<()>,Status>{
    let token=match req.metadata().get("authorization"){
        Some(token)=>token.to_str(),
        None=>return Err(Status::unauthenticated("Token not found"))
    };
    // do some validation with token here ...
    Ok(req)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let daemon = Daemon::default();
    let ser = VirtualFileSystemServer::with_interceptor(daemon,interceptor);
    let cert = include_str!("../server.pem");
    let key = include_str!("../server.key");
    let id = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());

    println!("Server listening on {}", addr);
    let s = include_str!("../my_ca.pem");
    let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
    let tls = tonic::transport::ServerTlsConfig::new()
        .identity(id)
        .client_ca_root(ca);
    Server::builder()
        .tls_config(tls)?
        .add_service(ser)
        .serve(addr)
        .await?;

    Ok(())
}
