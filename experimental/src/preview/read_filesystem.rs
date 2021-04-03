use std::{
    iter,
    path::Path
};

use crate::{
    filesystem::{
        Metadata,
        MetadataExt,
        ReadDir,
    },
    FileSystemError,
    ReadFileSystem,
    Result
};
use crate::preview::node::kind::PreviewNodeKind;

use super::{
    Preview,
    PreviewNode
};

impl ReadFileSystem for Preview {
    /// Errors :
    /// * The user lacks permissions to perform `metadata` call on `path`.
    /// * `path` does not exist.
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        let path = path.as_ref();
        if let Some(node) = self.root.find_at_path(path) {
            if node.is_deleted(){
                Err(FileSystemError::Custom(String::from("Path does not exists")))
            } else {
                node.into_virtual_metadata()
            }
        } else if path.exists() {
            path.metadata()?.into_virtual_metadata()
        } else {
            Err(FileSystemError::Custom(String::from("Path does not exists")))
        }
    }

    /// Errors :
    /// * The provided `path` doesn't exist.
    /// * The process lacks permissions to view the contents.
    /// * The `path` points at a non-directory file.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        let path = path.as_ref();
        if let Some(node) = self.root.find_at_path(path) {
            if node.is_deleted(){
                Err(FileSystemError::Custom(String::from("Path does not exists")))
            } else if let Some(children) = node.children() {
                let mut v : Vec<PreviewNode> = children.iter().cloned().collect();
                v.sort();
                Ok(ReadDir::new(path, v))
            } else {
                Err(FileSystemError::Custom(String::from("Not a directory")))
            }
        } else if path.exists() {
            if path.is_dir() {
                Ok(ReadDir::new(path, Vec::new()))
            } else {
                Err(FileSystemError::Custom(String::from("Not a directory")))
            }
        } else {
            Err(FileSystemError::Custom(String::from("Path does not exists")))
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashSet,
        path::PathBuf
    };

    use crate::{
        filesystem::{FileTypeExt, PathExt},
        sample::*
    };

    use super::*;

    #[test]
    fn read_dir_preview_iso_with_real() {
        let chroot = Chroot::new("read_dir_preview_iso_with_real");
        let chroot_path = chroot.init_simple();

        let preview = Preview::default();

        let real_read_dir_path_set : HashSet<PathBuf> = chroot_path.read_dir().unwrap().map(|dir_entry| dir_entry.unwrap().path()).collect();
        let preview_read_dir_path_set : HashSet<PathBuf> = chroot_path.preview_read_dir(&preview).unwrap().map(|dir_entry| dir_entry.unwrap().path()).collect();

        assert_eq!(real_read_dir_path_set, preview_read_dir_path_set);
        chroot.clean();
    }

    #[test]
    fn dir_metadata_iso_with_real() {
        let chroot = Chroot::new("dir_metadata_iso_with_real");
        let chroot_path = chroot.init_simple();

        let preview = Preview::default();

        let real_metadata = chroot_path.join("RDIR").metadata().unwrap();
        let preview_metadata = chroot_path.join("RDIR").preview_metadata(&preview).unwrap();

        assert_eq!(real_metadata.is_dir(), preview_metadata.is_dir());
        assert_eq!(real_metadata.is_file(), preview_metadata.is_file());
        assert_eq!(real_metadata.file_type().into_virtual_file_type().unwrap(), preview_metadata.file_type());

        chroot.clean();
    }

    #[test]
    fn file_metadata_iso_with_real() {
        let chroot = Chroot::new("file_metadata_iso_with_real");
        let chroot_path = chroot.init_simple();

        let preview = Preview::default();

        let real_metadata = chroot_path.join("RDIR/RFILEA").metadata().unwrap();
        let preview_metadata = chroot_path.join("RDIR/RFILEA").preview_metadata(&preview).unwrap();

        assert_eq!(real_metadata.is_dir(), preview_metadata.is_dir());
        assert_eq!(real_metadata.is_file(), preview_metadata.is_file());
        assert_eq!(real_metadata.file_type().into_virtual_file_type().unwrap(), preview_metadata.file_type());

        chroot.clean();
    }
}
