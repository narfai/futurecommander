pub struct FsError;
pub struct DirBuilder;  // A builder used to create directories in various manners.
impl DirBuilder {
    pub fn new() -> DirBuilder { unimplemented!(); }
    pub fn recursive(&mut self, recursive: bool) -> &mut Self { unimplemented!(); }
    pub fn create<P: AsRef<Path>>(&self, path: P) -> Result<(), FsError> { unimplemented!(); }
}
pub struct DirEntry;    // Entries returned by the ReadDir iterator.
impl DirEntry {
    pub fn path(&self) -> PathBuf { unimplemented!(); }
    pub fn metadata(&self) -> Result<Metadata, FsError>;
    pub fn file_type(&self) -> io::Result<FileType> { unimplemented!(); }
    pub fn file_name(&self) -> OsString { unimplemented!(); }
}
pub struct ReadDir;     // Iterator over the entries in a directory.
impl Iterator for ReadDir {
    type Item = Result<DirEntry, FsError>;
    fn next(&mut self) -> Option<Result<DirEntry, FsError>> { unimplemented!(); }
}
pub struct File;        // A reference to an open file on the filesystem.
impl File {
    // pub fn open<P: AsRef<Path>>(path: P) -> Result<File, FsError> { unimplemented!(); }
    pub fn create<P: AsRef<Path>>(path: P) -> Result<File, FsError> { unimplemented!(); }
    // pub fn with_options() -> OpenOptions { unimplemented!(); }
    // pub fn sync_all(&self) -> Result<(), FsError> { unimplemented!; }
    // pub fn sync_data(&self) -> Result<(), FsError> { unimplemented!; }
    // pub fn set_len(&self, size: u64) -> io::Result<()> { unimplemented!; }
    pub fn metadata(&self) -> Result<Metadata, FsError> { unimplemented!; }
    // pub fn try_clone(&self) -> Result<File, FsError> { unimplemented!; }
    // pub fn set_permissions(&self, perm: Permissions) -> Result<(), FsError> { unimplemented!; }
}
/*impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unimplemented!; }
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> { unimplemented!; }
    fn is_read_vectored(&self) -> bool { unimplemented!; }
    unsafe fn initializer(&self) -> Initializer { unimplemented!; }
}
impl Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unimplemented!; }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> { unimplemented!; }

    #[inline]
    fn is_write_vectored(&self) -> bool { unimplemented!; }

    fn flush(&mut self) -> io::Result<()> { unimplemented!; }
}
impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { unimplemented!; }
}
impl Read for &File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { unimplemented!; }
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> { unimplemented!; }
    fn is_read_vectored(&self) -> bool { unimplemented!; }
    unsafe fn initializer(&self) -> Initializer { unimplemented!; }
impl Write for &File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> { unimplemented!; }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize> { unimplemented!; }

    #[inline]
    fn is_write_vectored(&self) -> bool { unimplemented!; }

    fn flush(&mut self) -> io::Result<()> { unimplemented!; }
}
impl Seek for &File {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> { unimplemented!; }
}*/
pub struct FileType;    // A structure representing a type of file with accessors for each file type. It is returned by Metadata::file_type method.
impl FileType {
    pub fn is_dir(&self) -> bool { unimplemented!; }
    pub fn is_file(&self) -> bool { unimplemented!; }
    pub fn is_symlink(&self) -> bool { unimplemented!; }
}
pub struct Metadata;    // Metadata information about a file.
impl Metadata {
    pub fn file_type(&self) -> FileType { unimplemented!; }
    pub fn is_dir(&self) -> bool { unimplemented!; }
    pub fn is_file(&self) -> bool { unimplemented!; }
    pub fn len(&self) -> u64 { unimplemented!; }
    // pub fn permissions(&self) -> Permissions { unimplemented!; }
    // pub fn modified(&self) -> io::Result<SystemTime> { unimplemented!; }
    // pub fn accessed(&self) -> io::Result<SystemTime> { unimplemented!; }
    // pub fn created(&self) -> io::Result<SystemTime> { unimplemented!; }
}

/*
pub struct OpenOptions; // Options and flags which can be used to configure how a file is opened.
pub struct Permissions; // Representation of the various permissions on a file.
*/

pub fn metadata<P: AsRef<Path>>(path: P) -> Result<Metadata, FsError> { unimplemented!(); }  // Given a path, query the file system to get information about a file, directory, etc.
pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir, FsError> { unimplemented!(); }   // Returns an iterator over the entries within a directory.

pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<(), FsError>{ unimplemented!(); }       // Creates a new, empty directory at the provided path
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<(), FsError>{ unimplemented!(); }   // Recursively create a directory and all of its parent components if they are missing.
pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<u64, FsError>{ unimplemented!(); }    // Copies the contents of one file to another. This function will also copy the permission bits of the original file to the destination file.
pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), FsError>{ unimplemented!(); }   // Rename a file or directory to a new name, replacing the original file if to already exists.
pub fn remove_dir<P: AsRef<Path>>(path: P) -> Result<(), FsError>{ unimplemented!(); }       // Removes an empty directory.
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> Result<(), FsError>{ unimplemented!(); }   // Removes a directory at this path, after removing all its contents. Use carefully!
pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<(), FsError>{ unimplemented!(); }      // Removes a file from the filesystem.

/*
pub fn canonicalize(){ unimplemented!(); }     // Returns the canonical, absolute form of a path with all intermediate components normalized and symbolic links resolved.
pub fn hard_link(){ unimplemented!(); }        // Creates a new hard link on the filesystem.
pub fn read(){ unimplemented!(); }             // Read the entire contents of a file into a bytes vector.
pub fn read_link(){ unimplemented!(); }        // Reads a symbolic link, returning the file that the link points to.
pub fn read_to_string(){ unimplemented!(); }   // Read the entire contents of a file into a string.
pub fn set_permissions(){ unimplemented!(); }  // Changes the permissions found on a file or a directory.
pub fn symlink_metadata(){ unimplemented!(); } // Query the metadata about a file without following symlinks.
pub fn write(){ unimplemented!(); }            // Write a slice as the entire contents of a file.
*/