// use futures::stream::iter;
use tonic::Request;
use futures::stream;

use futurecommander_proto::vfs::{
    virtual_file_system_client::{VirtualFileSystemClient},
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

fn get_token() -> String {
    String::from("token")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cert=include_str!("../client.pem");
    let key=include_str!("../client.key");
    let id=tonic::transport::Identity::from_pem(cert.as_bytes(),key.as_bytes());
    let s=include_str!("../my_ca.pem");
    let ca=tonic::transport::Certificate::from_pem(s.as_bytes());
    let tls=tonic::transport::ClientTlsConfig::new().domain_name("localhost").identity(id).ca_certificate(ca);
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .tls_config(tls)?
        .connect()
        .await?;
    let token = get_token();
    let mut client = VirtualFileSystemClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert(
            "authorization",
            tonic::metadata::MetadataValue::from_str(&token).unwrap(),
        );
        Ok(req)
    });
     let request = tonic::Request::new(stream::iter(vec![
         RemoveNodeRequest {
            status:RequestStatus::Initiating as i32,
            recursive: true,
            path:String::from("A")
         },
         RemoveNodeRequest {
            status:RequestStatus::Initiating as i32,
            recursive: true,
            path:String::from("B")
         },
         RemoveNodeRequest {
            status:RequestStatus::Initiating as i32,
            recursive: true,
            path:String::from("C")
         },
     ]));

    let response = client.remove_node(request).await?;
    let mut inbound = response.into_inner();

     while let Some(res) = inbound.message().await? {
         println!("NOTE = {:?}", res);
     }

    Ok(())
}
