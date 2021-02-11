// use futures::stream::iter;
use tonic::Request;
use futures::stream;

use futurecommander_proto::vfs::{
    virtual_file_system_client::{VirtualFileSystemClient},
    Error,
    Entry,
    ListDirectoryRequest,
    ListDirectoryResponse,
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

// required for JWT
//fn get_token() -> String {
//    String::from("token")
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// required for TLS
//    let cert=include_str!("../tls/client.pem");
//    let key=include_str!("../tls/client.key");
//    let id=tonic::transport::Identity::from_pem(cert.as_bytes(),key.as_bytes());
//    let s=include_str!("../tls/my_ca.pem");
//    let ca=tonic::transport::Certificate::from_pem(s.as_bytes());
//    let tls=tonic::transport::ClientTlsConfig::new().domain_name("localhost").identity(id).ca_certificate(ca);

    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
// required for TLS
//        .tls_config(tls)?
        .connect()
        .await?;
    let mut client = VirtualFileSystemClient::new(channel);
// required for JWT
//    let token = get_token();
//    let mut client = VirtualFileSystemClient::with_interceptor(channel, move |mut req: Request<()>| {
//        req.metadata_mut().insert(
//            "authorization",
//            tonic::metadata::MetadataValue::from_str(&token).unwrap(),
//        );
//        Ok(req)
//    });
     let request = tonic::Request::new(ListDirectoryRequest {
        path:String::from("/home/narfai/current2/work_own_project/futurecommander/samples/static/A")
    });

    let response = client.list_directory(request).await?;
    let mut inbound = response.into_inner();

     while let Some(res) = inbound.message().await? {
         println!("NOTE = {:?}", res);
     }

    Ok(())
}
