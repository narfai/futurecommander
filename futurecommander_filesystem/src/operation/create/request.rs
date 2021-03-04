use std::path::{ PathBuf, Path };

use crate::{
    Kind,
    operation::{
        Request,
        create::serializable_kind::SerializableKind
    }
};


#[derive(Clone)]
pub struct CreateRequest {
    path: PathBuf,
    kind: SerializableKind
}

impl CreateRequest {
    pub fn new(path: PathBuf, kind: Kind) -> Self {
        CreateRequest { path, kind: kind.into() }
    }

    pub fn path(&self) -> &Path { &self.path }

    pub fn kind(&self) -> Kind { self.kind.into() }
}

impl Request for CreateRequest {}