use crate::{
    Result,
    FileSystemError
};
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FileType {
    File,
    Directory,
    Symlink
}

impl FileType {
    pub fn is_dir(&self) -> bool { matches!(self, FileType::Directory) }
    pub fn is_file(&self) -> bool { matches!(self, FileType::File) }
    pub fn is_symlink(&self) -> bool { matches!(self, FileType::Symlink) }
}

#[derive(Clone)]
pub struct Metadata {
    pub (in crate) file_type: FileType
}

impl Metadata {
    pub fn file_type(&self) -> FileType { self.file_type.clone() }
    pub fn is_dir(&self) -> bool { self.file_type().is_dir() }
    pub fn is_file(&self) -> bool { self.file_type().is_file() }
    // pub fn len(&self) -> u64 { unimplemented!(); }
    // pub fn permissions(&self) -> Permissions { unimplemented!; }
    // pub fn modified(&self) -> io::Result<SystemTime> { unimplemented!; }
    // pub fn accessed(&self) -> io::Result<SystemTime> { unimplemented!; }
    // pub fn created(&self) -> io::Result<SystemTime> { unimplemented!; }
}

pub trait MetadataExt {
    fn into_virtual_metadata(self) -> Result<Metadata>;
}

pub trait FileTypeExt {
    fn into_virtual_file_type(self) -> Result<FileType>;
}

use std::fs::Metadata as FsMetadata;
impl MetadataExt for FsMetadata {
    fn into_virtual_metadata(self) -> Result<Metadata> {
        Ok(
            Metadata {
                file_type: self.file_type().into_virtual_file_type()?
            }
        )
    }
}

use std::fs::FileType as FsFileType;
impl FileTypeExt for FsFileType {
    fn into_virtual_file_type(self) -> Result<FileType> {
        if self.is_symlink() {
            Ok(FileType::Symlink)
        } else if self.is_dir() {
            Ok(FileType::Directory)
        } else if self.is_file() {
            Ok(FileType::File)
        } else {
            Err(FileSystemError::Custom(String::from("Unknow file type")))
        }
    }
}

use crate::preview::{ Node, Kind };
impl MetadataExt for &Node {
    fn into_virtual_metadata(self) -> Result<Metadata> {
        Ok(
            Metadata {
                file_type: self.into_virtual_file_type()?
            }
        )
    }
}

impl FileTypeExt for &Node {
    fn into_virtual_file_type(self) -> Result<FileType> {
        match self.kind() {
            Kind::Symlink(_) => Ok(FileType::Symlink),
            Kind::Directory(_) => Ok(FileType::Directory),
            Kind::File(_) => Ok(FileType::File),
            Kind::Deleted => Err(FileSystemError::Custom(String::from("Delete file has no type")))
        }
    }
}