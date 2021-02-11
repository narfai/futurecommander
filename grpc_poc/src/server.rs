use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use futurecommander_proto::vfs::{
    virtual_file_system_server::{VirtualFileSystem,VirtualFileSystemServer},
    Error,
    Entry,
    ListDirectoryRequest,
    ListDirectoryResponse,
    list_directory_response::Response::Entry as ListEntry,
//    CreateNodeRequest,
//    CreateNodeResponse,
//    RemoveNodeRequest,
//    RemoveNodeResponse,
//    CopyNodeRequest,
//    CopyNodeResponse,
//    MoveNodeRequest,
//    MoveNodeResponse,
//    RequestStatus,
//    ResponseStatus,
};

#[derive(Default)]
pub struct Daemon {}

#[tonic::async_trait]
impl VirtualFileSystem for Daemon {
    type ListDirectoryStream = mpsc::Receiver<Result<ListDirectoryResponse, Status>>;
    async fn list_directory(
        &self,
        request: Request<ListDirectoryRequest>,
    ) -> Result<Response<Self::ListDirectoryStream>, Status> {        
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {            
            println!("Got a delete request {:?}", request);
            let req = request.into_inner();
            tx.send(Ok(ListDirectoryResponse {
                response: Some(
                    ListEntry(
                        Entry {
                            path: format!("{:?}/A", req.path),
                            name: String::from("A"),
                            is_dir: true,
                            is_file: false,
                            exists: true,
                            is_virtual: true
                        }
                    )
                )
            })).await;
            tx.send(Ok(ListDirectoryResponse {
                response: Some(
                    ListEntry(
                        Entry {
                            path: format!("{:?}/A", req.path),
                            name: String::from("A"),
                            is_dir: true,
                            is_file: false,
                            exists: true,
                            is_virtual: true
                        }
                    )
                )
            })).await;
        });
        Ok(Response::new(rx))
    }
}

// required for JWT
//fn interceptor(req:Request<()>)->Result<Request<()>,Status>{
//    let token=match req.metadata().get("authorization"){
//        Some(token)=>token.to_str(),
//        None=>return Err(Status::unauthenticated("Token not found"))
//    };
//    // do some validation with token here ...
//    Ok(req)
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let daemon = Daemon::default();

// Required for JWT
//    let ser = VirtualFileSystemServer::with_interceptor(daemon,interceptor);
    let ser = VirtualFileSystemServer::new(daemon);

// required for TLS
//    let cert = include_str!("../tls/server.pem");
//    let key = include_str!("../tls/server.key");
//    let id = tonic::transport::Identity::from_pem(cert.as_bytes(), key.as_bytes());

    println!("Server listening on {}", addr);
// required for TLS
//    let s = include_str!("../tls/my_ca.pem");
//    let ca = tonic::transport::Certificate::from_pem(s.as_bytes());
//    let tls = tonic::transport::ServerTlsConfig::new()
//        .identity(id)
//        .client_ca_root(ca);

    Server::builder()
// required for TLS
//        .tls_config(tls)?
        .add_service(ser)
        .serve(addr)
        .await?;

    Ok(())
}
