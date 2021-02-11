use std::{
    path::{ Path, PathBuf },
    sync::{Arc, Mutex}
};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

use futurecommander_filesystem::{
    Container,
    Kind,
    ReadableFileSystem,
    Entry as EntryTrait,
    tools::{ absolute }
};

use futurecommander_proto::vfs::{
    virtual_file_system_server::{VirtualFileSystem,VirtualFileSystemServer},
    Error,
    Entry,
    ListDirectoryRequest,
    ListDirectoryResponse,
    list_directory_response::Response::Entry as ListEntry,
    list_directory_response::Response::Error as ListError,
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

pub struct Daemon {
    container: Arc<Mutex<Container>>
}

impl Default for Daemon {
    fn default() -> Self {
        Daemon {
            container: Arc::new(Mutex::new(Container::new())),
        }
    }
}

#[tonic::async_trait]
impl VirtualFileSystem for Daemon {
    type ListDirectoryStream = mpsc::Receiver<Result<ListDirectoryResponse, Status>>;
    async fn list_directory(
        &self,
        request: Request<ListDirectoryRequest>,
    ) -> Result<Response<Self::ListDirectoryStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);

        println!("Got a list request {:?}", request);
        let container = self.container.lock().unwrap();
        let collection = container.read_dir(&Path::new(&request.into_inner().path)).unwrap();
        tokio::spawn(async move {
            for child in collection.sort().into_iter() {
                tx.send(Ok(ListDirectoryResponse {
                    response: Some(
                        ListEntry(
                            Entry {
                                path: child.path().to_string_lossy().to_string(),
                                name: child.name().unwrap().to_string_lossy().to_string(),
                                is_dir: child.is_dir(),
                                is_file: child.is_file(),
                                is_virtual: child.is_virtual()
                            }
                        )
                    )
                })).await;
            }
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
