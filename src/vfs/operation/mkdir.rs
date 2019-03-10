use crate::VirtualPath;
use crate::VirtualFileSystem;
use std::path::Path;

pub fn mkdir(vfs: &mut VirtualFileSystem, identity: &Path) {
    vfs.mkdir(identity);
}
