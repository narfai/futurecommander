// use futures::stream::iter;
use std::io::{Write, stdout, stdin};

use tonic::Request;
use futures::stream;

use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;

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

const HORZ_BOUNDARY: &'static str = "─";
const VERT_BOUNDARY: &'static str = "│";
const TOP_LEFT_CORNER: &'static str = "┌";
const TOP_RIGHT_CORNER: &'static str = "┐";
const BOTTOM_LEFT_CORNER: &'static str = "└";
const BOTTOM_RIGHT_CORNER: &'static str = "┘";

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

    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w - 2);
    let termheight = termsize.map(|(_,h)| h - 2);

    println!("{:?}", termsize);
    println!("{:?}", termwidth);
    println!("{:?}", termheight);

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}q to exit. Click, click, click!", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
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
                    },
                    _ => (),
                }
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }

    Ok(())
}
