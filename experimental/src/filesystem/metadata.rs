
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
    file_type: FileType
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