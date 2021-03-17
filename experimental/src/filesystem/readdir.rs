use std::{
    path::{ PathBuf },
    ffi::{ OsString }
};

use crate::{
    Result
};

use self::super::{
    FileSystemError,
    Metadata,
    FileType
};

pub struct ReadDir;     // Iterator over the entries in a directory.
impl Iterator for ReadDir {
    type Item = Result<DirEntry>;
    fn next(&mut self) -> Option<Result<DirEntry>> { unimplemented!(); }
}

pub struct DirEntry {
    path: PathBuf,
    name: OsString,
    metadata: Metadata
}

impl DirEntry {
    pub fn path(&self) -> PathBuf { self.path.to_path_buf() }
    pub fn metadata(&self) -> Result<Metadata> { Ok(self.metadata.clone()) }
    pub fn file_type(&self) -> Result<FileType> { Ok(self.metadata.file_type()) }
    pub fn file_name(&self) -> OsString { self.name.clone() }
}

