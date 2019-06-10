mod utils;
mod errors;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::{ prelude::* };
use js_sys::{ Function };
use bytes::{
    BytesMut,
    BufMut
};

use byteorder::{
    ReadBytesExt
};
use std::{
    net::{ TcpStream },
    path::{ PathBuf }
};

use tokio_codec::{
    Decoder
};

use futurecommander_protocol::{
    PacketCodec,
    Packet,
    Header,
    message::{
        Message,
        DirectoryOpen,
        DirectoryRead
    }
};

#[wasm_bindgen]
pub struct MessageDelta {
    packet: Option<Packet>,
    index: usize
}

#[wasm_bindgen]
impl MessageDelta {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn header(&self) -> JsValue {
        if let Some(packet) = &self.packet {
            match JsValue::from_serde(&packet.header()) {
                Ok(value) => value,
                Err(_) => JsValue::NULL
            }
        } else {
            JsValue::NULL
        }
    }

    pub fn len(&self) -> JsValue {
        if let Some(packet) = &self.packet {
            JsValue::from_f64(packet.length() as f64)
        } else {
            JsValue::NULL
        }
    }

    pub fn parse(&self) -> Result<JsValue, JsValue> {
        if let Some(packet) = &self.packet {
            match packet.header() {
                Header::DirectoryRead => JsValue::from_serde(&packet.parse::<DirectoryRead>().unwrap())
                    .map_err(|error| errors::AddonError::from(error).into())
                ,
                _ => Err(errors::AddonError::InvalidArgument("Unsupported header".to_string()).into())
            }
        } else {
            Ok(JsValue::NULL)
        }

    }
}

#[wasm_bindgen]
pub struct ProtocolCodec {
    codec : PacketCodec
}

#[wasm_bindgen]
impl ProtocolCodec {

    #[wasm_bindgen(constructor)]
    pub fn new() -> ProtocolCodec {
        ProtocolCodec {
            codec: PacketCodec::default()
        }
    }

    pub fn read_dir(&self) -> Result<Box<[u8]>, JsValue> {
        // TODO encode with context as previous poc
        let message = DirectoryOpen { path: PathBuf::from("/tmp2") };
        message.encode()
            .and_then(|packet| {
                let mut buffer = BytesMut::new();
                packet.write(&mut buffer)
                    .and_then(|_|
                        Ok(buffer.freeze().to_vec())
                    )
            })
            .map_err(|error| errors::AddonError::from(error).into())
            .map(|raw| raw.into_boxed_slice())
    }

    pub fn decode(&mut self, read_buffer: &[u8]) -> Result<MessageDelta, JsValue> {
        let mut codec = PacketCodec::default();
        codec.decode(&mut BytesMut::from(read_buffer))
            .map_err(|error| errors::AddonError::from(error).into())
            .and_then(|maybe| {
                Ok(
                    MessageDelta {
                        packet: maybe,
                        index: codec.index()
                    })
            })

    }

//    pub fn listen(&self, closure: &Function){
//        let this = JsValue::NULL;
//        let test = "from_rust";
////        let _ = closure.call1(&this, &JsValue::from(test));
////        let client = Client::new(
////            tools::parse_address(None, None)
////        );
//
//        let listener = TcpStream::connect("127.0.0.1:7842").unwrap();
//
//        // accept connections and process them serially
//        for stream in listener.incoming() {
//            handle_client(stream?);
//        }
//        Ok(())
//    }
}
